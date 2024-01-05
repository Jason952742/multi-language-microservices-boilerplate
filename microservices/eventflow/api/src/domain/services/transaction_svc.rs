use chrono::Utc;
use tonic::Status;
use uuid::Uuid;
use shared::{GrpcStatusTool, uuid_to_base64};
use crate::domain::aggregates::account_ar::{Account};
use crate::domain::commands::eventflow_cmd::EventflowEvent;
use crate::domain::entities::enums::{TransactionStatus, TransactionType};
use crate::domain::entities::{transaction};
use crate::domain::entities::valobj::{Payment, User};
use crate::domain::queries::account_qry::AccountQuery;
use crate::domain::services::AccountServices;
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
        let account_es = AccountServices::create_event(&account_id, &user_id, &transaction_id).await;

        EventSourceDbMutation::create_eventsource(Account::TABLE_NAME, account_es.clone()).await.unwrap();

        TransactionDbMutation::update_transaction(
            transaction_id,
            TransactionStatus::Completed,
            vec![format!("Account:{:?}", account_es.id)],
            None,
        ).await.map_err(|e| GrpcStatusTool::invalid(e.to_string().as_str()))?;

        let sub_end_date = Utc::now();
        let user = User { user_id, user_name, member_id, sub_end_date, account_id, refer_code, ..Default::default() };

        Ok(EventflowEvent::Created { user })
    }

    pub async fn account_deposit(user_id: Uuid, account_id: Uuid, payment: Payment) -> Result<EventflowEvent, Status> {
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

    pub async fn account_withdraw(user_id: Uuid, account_id: Uuid, payment: Payment) -> Result<EventflowEvent, Status> {
        todo!()
    }

    pub async fn member_subscribe(user_id: Uuid, member_id: Uuid, payments: Vec<Payment>, duration: i32) -> Result<EventflowEvent, Status> {
        todo!()
    }
}
