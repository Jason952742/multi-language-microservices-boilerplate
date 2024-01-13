use std::ops::Add;
use chrono::{Duration, Utc};
use tonic::Status;
use uuid::Uuid;
use shared::utils::{GrpcStatusTool, uuid_to_base64};
use crate::application::events::publishers::{AccountPub, MemberPub, ReferralPub};
use crate::domain::aggregates::account_ar::{Account};
use crate::domain::aggregates::member_ar::Member;
use crate::domain::commands::eventflow_cmd::EventflowEvent;
use crate::domain::entities::enums::{CurrencyType, TransactionStatus, TransactionType};
use crate::domain::entities::{transaction};
use crate::domain::entities::valobj::{Payment, User};
use crate::domain::messages::{AccountCreatedMsg, MemberCreatedMsg, MemberReferralMsg};
use crate::domain::queries::account_qry::AccountQuery;
use crate::domain::queries::member_qry::MemberQuery;
use crate::domain::queries::referral_qry::ReferralQuery;
use crate::domain::services::{AccountServices, MemberServices, ReferralServices};
use crate::infra::repositories::eventsource_mutation::EventSourceDbMutation;
use crate::infra::repositories::transaction_mutation::TransactionDbMutation;

pub struct TransactionService;

impl TransactionService {
    pub async fn create_user(user_id: Uuid, user_name: String, referrer_id: Option<Uuid>, referrer_code: &Option<String>, payload: String) -> Result<EventflowEvent, Status> {
        let txn_id = Uuid::new_v4();
        let member_id = Uuid::new_v4();
        let account_id = Uuid::new_v4();
        let referral_code = uuid_to_base64(Uuid::new_v4());

        // start transaction
        TransactionDbMutation::create_transaction(
            transaction::Model {
                id: txn_id.clone(),
                transaction_type: TransactionType::UserCreate,
                user_id: user_id.clone(),
                created_at: Utc::now(),
                updated_at: Utc::now(),
                payload,
                ..Default::default()
            }
        ).await.map_err(|e| GrpcStatusTool::invalid(e.to_string().as_str()))?;

        // account event
        let mut events = vec![];
        let account_es = AccountServices::create_event(&account_id, &user_id, &txn_id).await;
        events.push(account_es);
        let member_es = MemberServices::register_event(&member_id, &user_id, user_name.clone(), &txn_id).await;
        events.push(member_es);
        let referral_es = ReferralServices::create_referral_event(&user_id, &referral_code, &referrer_id, referrer_code, &txn_id).await;
        events.push(referral_es);
        if let Some(id) = referrer_id.clone() {
            let referrer = ReferralQuery::load(id).await?;
            if let Some(r) = referrer {
                let referrer_es = ReferralServices::user_registered_event(&r, user_id).await;
                events.push(referrer_es);
            }
        }
        let event_ids = events.clone().into_iter().map(|e| format!("{:?}:{:?}", e.aggregate_type, e.id)).collect();

        // batch insert events
        match EventSourceDbMutation::batch_eventsource(events).await {
            Ok(_) => {
                // transaction successfully
                TransactionDbMutation::update_transaction(txn_id, TransactionStatus::Completed, event_ids, None)
                    .await.map_err(|e| GrpcStatusTool::invalid(e.to_string().as_str()))?;

                let sub_end_date = Utc::now().add(Duration::hours(24));
                let user = User {
                    user_id,
                    user_name: user_name.clone(),
                    member_id,
                    sub_end_date: sub_end_date.clone(),
                    account_id,
                    referral_code: referral_code.clone(),
                    ..Default::default()
                };

                // publish mq messages
                ReferralPub::publish_referral(MemberReferralMsg { user_id, user_name: user_name.clone(), member_id, referral_code, referrer_id })
                    .await.map_err(|e| GrpcStatusTool::invalid(e.to_string().as_str()))?;
                MemberPub::publish_member(MemberCreatedMsg { user_id, user_name, member_id, sub_end_date })
                    .await.map_err(|e| GrpcStatusTool::invalid(e.to_string().as_str()))?;
                AccountPub::publish_account(AccountCreatedMsg { user_id, account_id, ccy_type: CurrencyType::EUR })
                    .await.map_err(|e| GrpcStatusTool::invalid(e.to_string().as_str()))?;

                Ok(EventflowEvent::Created { user })
            }
            Err(_) => {
                // todo: rollback transaction
                Err(Status::internal("Transaction failed"))
            }
        }
    }

    pub async fn account_deposit(account_id: Uuid, payment: Payment) -> Result<EventflowEvent, Status> {
        let account = AccountQuery::load(account_id).await.map_err(|e| GrpcStatusTool::invalid(e.to_string().as_str()))?;
        match account {
            None => Err(Status::not_found("account not found")),
            Some(a) => {
                let (es, balance) = AccountServices::deposit_event(&a, payment).await;
                EventSourceDbMutation::create_eventsource(Account::TABLE_NAME, es.clone()).await.unwrap();
                Ok(EventflowEvent::AccountDeposited { account_id, balance })
            }
        }
    }

    pub async fn account_withdraw(account_id: Uuid, payment: Payment) -> Result<EventflowEvent, Status> {
        let account = AccountQuery::load(account_id).await.map_err(|e| GrpcStatusTool::invalid(e.to_string().as_str()))?;
        match account {
            None => Err(Status::not_found("account not found")),
            Some(a) => {
                let (es, balance) = AccountServices::withdraw_event(&a, payment).await;
                EventSourceDbMutation::create_eventsource(Account::TABLE_NAME, es.clone()).await.unwrap();
                Ok(EventflowEvent::AccountWithdrew { account_id, balance })
            }
        }
    }

    pub async fn member_subscribe(member_id: Uuid, payments: Vec<Payment>, duration: i64) -> Result<EventflowEvent, Status> {
        let member = MemberQuery::load(member_id).await.map_err(|e| GrpcStatusTool::invalid(e.to_string().as_str()))?;

        match member {
            None => Err(Status::not_found("member not found")),
            Some(m) => {
                let (es, end_date) = MemberServices::subscribe_event(&m, payments, duration).await;
                EventSourceDbMutation::create_eventsource(Member::TABLE_NAME, es.clone()).await.unwrap();
                Ok(EventflowEvent::MemberSubscribed { member_id, end_date })
            }
        }
    }
}
