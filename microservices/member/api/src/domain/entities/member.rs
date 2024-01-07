use sea_orm::entity::prelude::*;
use rust_decimal::Decimal;
use uuid::Uuid;
use serde::{Deserialize, Serialize};
use crate::domain::entities::enums::{MemberStatus, MemberType};

#[derive(Default, Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "members")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    #[sea_orm(indexed, unique)]
    pub user_id: Uuid,
    #[sea_orm(indexed, unique)]
    pub user_name: String,
    
    pub status: MemberStatus,
    pub member_type: MemberType,
    pub credit_score: Decimal,
    pub point: i32,
    pub level: i32,
    pub active: bool,
    pub description: String,

    #[serde(skip_deserializing)]
    pub creator: Option<Uuid>,
    #[serde(skip_deserializing)]
    pub modifier: Option<Uuid>,
    #[serde(skip_deserializing)]
    pub check_sum: Option<String>,
    pub region: Option<String>,
    pub group_id: Option<String>,
    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
    #[sea_orm(default_value = true)]
    pub enabled: bool,
    #[sea_orm(default_value = 0)]
    pub version: i32,
    #[sea_orm(default_value = false, indexed)]
    #[serde(skip_deserializing)]
    pub deleted: bool,
    #[sea_orm(nullable)]
    #[serde(skip_deserializing)]
    pub deleted_at: Option<DateTimeUtc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
