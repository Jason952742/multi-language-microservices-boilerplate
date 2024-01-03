use sea_orm::*;
use uuid::Uuid;
use shared::postgres::PgPool;
use crate::domain::entities::member;
use crate::domain::entities::enums::{MemberStatus, MemberType};

pub struct MemberOrmQuery;

impl MemberOrmQuery {

    pub async fn get_member_by_id(id: Uuid) -> Result<Option<member::Model>, DbErr> {
        let db: &DbConn = PgPool::conn().await;
        member::Entity::find_by_id(id).one(db).await
    }

    pub async fn get_member_by_user_id(user_id: Uuid) -> Result<Option<member::Model>, DbErr> {
        let db: &DbConn = PgPool::conn().await;
        member::Entity::find()
            .filter(member::Column::UserId.eq(user_id))
            .one(db)
            .await
    }

    pub async fn find_members_in_page(
        page: u64, per_page: u64, status: Option<MemberStatus>, member_type: Option<MemberType>, level: Option<i32>,
    ) -> Result<(Vec<member::Model>, u64), DbErr> {
        let db: &DbConn = PgPool::conn().await;
        // Setup paginator
        let mut select = member::Entity::find()
            .filter(member::Column::Deleted.eq(false));

        if let Some(x) = status {
            select = select.filter(member::Column::Status.eq(x))
        }

        if let Some(x) = member_type {
            select = select.filter(member::Column::MemberType.eq(x))
        }

        if let Some(x) = level {
            select = select.filter(member::Column::Level.eq(x))
        }

        let paginator = select.order_by_asc(member::Column::CreatedAt)
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