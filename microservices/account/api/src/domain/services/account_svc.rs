use crate::domain::commands::account_cmd::AccountEvent;
use crate::domain::entities::account;
use crate::domain::entities::enums::{AccountType, CurrencyType};
use crate::infra::repositories::account_mutation::AccountDbMutation;
use crate::infra::repositories::account_query::AccountDbQuery;
use shared::utils::GrpcStatusTool;
use tonic::Status;
use uuid::Uuid;

pub struct AccountService;

impl AccountService {
  pub async fn create_account(id: Uuid, user_id: Uuid, ccy_type: CurrencyType) -> Result<AccountEvent, Status> {
    match AccountDbQuery::get_account_by_id(id).await.map_err(|e| GrpcStatusTool::db_error(e))? {
      Some(_) => Err(Status::already_exists("account already exists")),
      None => {
        let form_data: account::Model = account::Model { id, user_id, ccy_type, ..Default::default() };
        match AccountDbMutation::create_account(form_data).await {
          Ok(id) => Ok(AccountEvent::Created { id }),
          Err(_) => Err(Status::internal("Failed to create")),
        }
      }
    }
  }

  pub async fn update_account(user_id: Uuid, account_type: AccountType, account_name: String, description: String) -> Result<AccountEvent, Status> {
    match AccountDbMutation::update_account(user_id, account_type, account_name, description).await {
      Ok(_) => Ok(AccountEvent::Updated),
      Err(e) => Err(GrpcStatusTool::db_error(e)),
    }
  }

  pub async fn disabled_account(user_id: Uuid) -> Result<AccountEvent, Status> {
    match AccountDbMutation::disable_account(user_id).await {
      Ok(_) => Ok(AccountEvent::Disabled),
      Err(e) => Err(GrpcStatusTool::db_error(e)),
    }
  }
}
