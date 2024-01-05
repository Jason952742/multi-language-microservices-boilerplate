use rust_decimal_macros::dec;
use tonic::Status;
use uuid::Uuid;
use shared::{GrpcStatusTool, uuid_to_base64};
use crate::domain::commands::eventflow_cmd::EventflowEvent;
use crate::domain::entities::enums::{MemberType, TransactionType};
use crate::domain::entities::transaction;
use crate::domain::entities::valobj::User;
use crate::infra::repositories::transaction_mutation::TransactionDbMutation;

pub struct TransactionService;

impl TransactionService {
    pub async fn create_user(user_id: Uuid, user_name: String, data: String) -> Result<EventflowEvent, Status> {
        let transaction_id = Uuid::new_v4();
        let member_id = Uuid::new_v4();
        let account_id = Uuid::new_v4();
        let refer_code = uuid_to_base64(Uuid::new_v4());

        let form_data = transaction::Model {
            id: transaction_id.clone(),
            transaction_type: TransactionType::UserCreate,
            user_id: user_id.clone(),
            data,
            ..Default::default()
        };

        TransactionDbMutation::create_transaction(form_data).await.map_err(|e| GrpcStatusTool::invalid(e.to_string().as_str()))?;
        
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
