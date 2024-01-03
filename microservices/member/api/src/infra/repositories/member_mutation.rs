use uuid::Uuid;
use sea_orm::*;
use chrono::Local;
use shared::postgres::PgPool;
use crate::domain::entities::enums::MemberType;
use crate::domain::entities::member;

pub struct MemberOrmMutation;

impl MemberOrmMutation {
    pub async fn create_member(form_data: member::Model) -> Result<member::Model, DbErr> {
        let db: &DbConn = PgPool::conn().await;
        create(form_data).insert(db).await
    }

    pub async fn update_member(user_id: Uuid, member_type: MemberType, level: i32, active: bool, description: String) -> Result<member::Model, DbErr> {
        let db: &DbConn = PgPool::conn().await;
        let active_model = update(user_id).await?;

        member::ActiveModel {
            member_type: Set(member_type.to_owned()),
            level: Set(level.to_owned()),
            active: Set(active.to_owned()),
            description: Set(description.to_owned()),
            ..active_model
        }.update(db).await
    }

    pub async fn disable_member(user_id: Uuid) -> Result<member::Model, DbErr> {
        let db: &DbConn = PgPool::conn().await;

        let active_model = update(user_id).await?;

        member::ActiveModel {
            enabled: Set(false),
            ..active_model
        }.update(db).await
    }

    pub async fn _soft_delete(user_id: Uuid) -> Result<member::Model, DbErr> {
        let db: &DbConn = PgPool::conn().await;
        let active_model = update(user_id).await?;

        // set deleted
        member::ActiveModel {
            deleted: Set(true),
            deleted_at: Set(Some(Local::now().naive_local())),
            ..active_model
        }.update(db).await
    }
}

fn create(model: member::Model) -> member::ActiveModel {
    let active_model = member::ActiveModel::from(model);
    member::ActiveModel { enabled: Set(true), created_at: Set(Local::now().naive_local()), updated_at: Set(Local::now().naive_local()), ..active_model }
}

async fn update(user_id: Uuid) -> Result<member::ActiveModel, DbErr> {
    let db: &DbConn = PgPool::conn().await;
    let active_model: member::ActiveModel = member::Entity::find()
        .filter(member::Column::UserId.eq(user_id.to_owned()))
        .one(db).await?
        .ok_or(DbErr::RecordNotFound(format!("Cannot find user {:?}.", user_id)))
        .map(Into::into)?;

    Ok(member::ActiveModel { version: Set(active_model.version.unwrap() + 1), updated_at: Set(Local::now().naive_local()), ..active_model })
}

#[tokio::test]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}