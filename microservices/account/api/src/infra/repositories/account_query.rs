use sea_orm::*;
use uuid::Uuid;
use shared::datasource::mariadb::MariaPool;
use crate::domain::entities::account;
use crate::domain::entities::enums::{AccountStatus, AccountType, CurrencyType};

pub struct AccountDbQuery;

impl AccountDbQuery {

    pub async fn get_account_by_id(id: Uuid) -> Result<Option<account::Model>, DbErr> {
        let db: &DbConn = MariaPool::conn().await;
        account::Entity::find_by_id(id).one(db).await
    }

    pub async fn get_accounts_by_user_id(user_id: Uuid) -> Result<Vec<account::Model>, DbErr> {
        let db: &DbConn = MariaPool::conn().await;
        account::Entity::find()
            .filter(account::Column::UserId.eq(user_id))
            .all(db)
            .await
    }

    pub async fn find_account_in_page(
        page: u64, per_page: u64, status: Option<AccountStatus>, ccy_type: Option<CurrencyType>, account_type: Option<AccountType>,
    ) -> Result<(Vec<account::Model>, u64), DbErr> {
        let db: &DbConn = MariaPool::conn().await;
        // Setup paginator
        let mut select = account::Entity::find()
            .filter(account::Column::Deleted.eq(false));

        if let Some(x) = status {
            select = select.filter(account::Column::Status.eq(x))
        }

        if let Some(x) = ccy_type {
            select = select.filter(account::Column::CcyType.eq(x))
        }

        if let Some(x) = account_type {
            select = select.filter(account::Column::AccountType.eq(x))
        }

        let paginator = select.order_by_asc(account::Column::CreatedAt)
            .paginate(db, per_page);

        let num_pages = paginator.num_pages().await?;

        // Fetch paginated posts
        paginator.fetch_page(page - 1).await.map(|p| (p, num_pages))
    }
}

#[tokio::test]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}