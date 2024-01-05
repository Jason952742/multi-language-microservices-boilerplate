use chrono::Utc;
use rust_decimal_macros::dec;
use serde_json::Value;
use tonic::Status;
use uuid::Uuid;
use shared::{GrpcStatusTool, uuid_to_base64};
use crate::domain::aggregates::account_ar::{Account, AccountCommand};
use crate::domain::commands::eventflow_cmd::EventflowEvent;
use crate::domain::entities::enums::{AggregateType, CurrencyType, MemberType, TransactionStatus, TransactionType};
use crate::domain::entities::{eventsource, transaction};
use crate::domain::entities::valobj::User;
use crate::infra::repositories::eventsource_mutation::EventSourceDbMutation;
use crate::infra::repositories::transaction_mutation::TransactionDbMutation;

pub struct TransactionService;

impl TransactionService {
    pub async fn create_user(user_id: Uuid, user_name: String, payload: String) -> Result<EventflowEvent, Status> {
        let transaction_id = Uuid::new_v4();
        let member_id = Uuid::new_v4();
        let account_id = Uuid::new_v4();
        let refer_code = uuid_to_base64(Uuid::new_v4());

        // start transaction
        TransactionDbMutation::create_transaction(
            transaction::Model {
                id: transaction_id.clone(),
                transaction_type: TransactionType::UserCreate,
                user_id: user_id.clone(),
                payload,
                ..Default::default()
            }
        ).await.map_err(|e| GrpcStatusTool::invalid(e.to_string().as_str()))?;

        // account event
        let account = Account::new(&account_id);
        let account_cmd = AccountCommand::OpenAccount { account_id: account_id.clone(), user_id, currency_type: CurrencyType::EUR };
        let account_event = account.handle(account_cmd).await.unwrap();
        let payload: Value = account_event.clone().into();
        let account_es_id = Uuid::new_v4();
        let account_es = eventsource::Model {
            id: account_es_id,
            txn_id: Some(transaction_id),
            aggregate_id: *&account_id,
            aggregate_type: AggregateType::Account,
            sequence: Utc::now().timestamp(),
            event_type: account_event.event_type(),
            event_version: account_event.event_version(),
            payload: payload.to_string(),
            created_at: Utc::now(),
            ..Default::default()
        };

        EventSourceDbMutation::create_eventsource(Account::TABLE_NAME, account_es).await.unwrap();

        TransactionDbMutation::update_transaction(
            transaction_id,
            TransactionStatus::Completed,
            vec![
                format!("Account:{:?}", account_es_id)
            ],
            None
        ).await.map_err(|e| GrpcStatusTool::invalid(e.to_string().as_str()))?;

        let user = User {
            user_id,
            user_name,
            member_id,
            member_type: MemberType::Wood,
            subscription_end_date: Default::default(),
            account_id,
            account_balance: dec!(0.0),
            refer_code,
            created_at: Default::default(),
        };

        Ok(EventflowEvent::Created { user })
    }

    pub async fn update_referral(user_id: Uuid, member_type: MemberType, level: i32, active: bool, description: String) -> Result<EventflowEvent, Status> {
        todo!()
    }

    pub async fn bind_referral(user_id: Uuid, referral_id: Uuid) -> Result<EventflowEvent, Status> {
        todo!()
    }
}
