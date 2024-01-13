use crate::domain::entities::account;
use crate::domain::entities::enums::AccountType;
use chrono::Utc;
use sea_orm::*;
use shared::datasource::mariadb::MariaPool;
use uuid::Uuid;

pub struct AccountDbMutation;

impl AccountDbMutation {
  pub async fn create_account(form_data: account::Model) -> Result<Uuid, DbErr> {
    let db: &DbConn = MariaPool::conn().await;
    let inserted = create(form_data.clone()).insert(db).await.expect("model save failed");
    Ok(inserted.id)
  }

  pub async fn update_account(user_id: Uuid, account_type: AccountType, account_name: String, description: String) -> Result<account::Model, DbErr> {
    let db: &DbConn = MariaPool::conn().await;
    let active_model = update(user_id).await?;

    account::ActiveModel { account_type: Set(account_type.to_owned()), account_name: Set(account_name.to_owned()), description: Set(description.to_owned()), ..active_model }
      .update(db)
      .await
  }

  pub async fn disable_account(user_id: Uuid) -> Result<account::Model, DbErr> {
    let db: &DbConn = MariaPool::conn().await;

    let active_model = update(user_id).await?;

    account::ActiveModel { enabled: Set(false), ..active_model }.update(db).await
  }

  pub async fn _soft_delete(user_id: Uuid) -> Result<account::Model, DbErr> {
    let db: &DbConn = MariaPool::conn().await;
    let active_model = update(user_id).await?;

    // set deleted
    account::ActiveModel { deleted: Set(true), deleted_at: Set(Some(Utc::now())), ..active_model }.update(db).await
  }
}

fn create(model: account::Model) -> account::ActiveModel {
  let active_model = account::ActiveModel::from(model);
  account::ActiveModel { enabled: Set(true), created_at: Set(Utc::now()), updated_at: Set(Utc::now()), ..active_model }
}

async fn update(id: Uuid) -> Result<account::ActiveModel, DbErr> {
  let db: &DbConn = MariaPool::conn().await;
  let active_model: account::ActiveModel = account::Entity::find_by_id(id)
    .one(db)
    .await?
    .ok_or(DbErr::RecordNotFound(format!("Cannot find user {:?}.", id)))
    .map(Into::into)?;

  Ok(account::ActiveModel { version: Set(active_model.version.unwrap() + 1), updated_at: Set(Utc::now()), ..active_model })
}

#[tokio::test]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let account = account::Model { ..Default::default() };

  AccountDbMutation::create_account(account).await?;

  Ok(())
}
