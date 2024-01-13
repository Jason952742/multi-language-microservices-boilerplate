use sea_orm::entity::prelude::*;
use rust_decimal::Decimal;
use uuid::Uuid;
use serde::{Deserialize, Serialize};
use crate::domain::entities::enums::{AccountStatus, AccountType, CurrencyType};

#[derive(Default, Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "accounts")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    #[sea_orm(indexed, unique)]
    pub user_id: Uuid,
    #[sea_orm(indexed, unique)]
    pub account_name: String,

    pub status: AccountStatus,
    pub account_type: AccountType,
    pub ccy_type: CurrencyType,
    pub deposit_count: i32,
    pub total_deposit: Decimal,
    pub withdraw_count: i32,
    pub total_withdraw: Decimal,
    pub earn_count: i32,
    pub total_earn: Decimal,
    pub spend_count: i32,
    pub total_spend: Decimal,
    pub commission_count: i32,
    pub total_commission: Decimal,
    pub frozen_amount: Decimal,
    pub balance: Decimal,
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
