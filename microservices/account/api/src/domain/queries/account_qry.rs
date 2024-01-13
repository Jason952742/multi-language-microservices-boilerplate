use crate::domain::entities::account;
use crate::domain::entities::enums::{AccountStatus, AccountType, CurrencyType};
use crate::infra::repositories::account_query::AccountDbQuery;
use shared::utils::GrpcStatusTool;
use tonic::Status;
use uuid::Uuid;

pub struct AccountQuery;

impl AccountQuery {
  pub async fn get_accounts(
    page: u64,
    per_page: u64,
    status: Option<AccountStatus>,
    ccy_type: Option<CurrencyType>,
    account_type: Option<AccountType>,
  ) -> Result<(Vec<account::Model>, u64), Status> {
    AccountDbQuery::find_account_in_page(page, per_page, status, ccy_type, account_type)
      .await
      .map_err(|e| GrpcStatusTool::db_error(e))
  }

  pub async fn get_accounts_by_user_id(user_id: Uuid) -> Result<Vec<account::Model>, Status> {
    AccountDbQuery::get_accounts_by_user_id(user_id).await.map_err(|e| GrpcStatusTool::db_error(e))
  }

  pub async fn get_account_by_id(id: Uuid) -> Result<Option<account::Model>, Status> {
    AccountDbQuery::get_account_by_id(id).await.map_err(|e| GrpcStatusTool::db_error(e))
  }
}
