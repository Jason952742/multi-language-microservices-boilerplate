use sea_orm::entity::prelude::*;
use serde_derive::{Deserialize, Serialize};
use strum_macros::EnumString;

#[derive(Default, Debug, Clone, PartialEq, Eq, EnumString, EnumIter, DeriveActiveEnum, Deserialize, Serialize)]
#[sea_orm(rs_type = "String", db_type = "String(Some(1))")]
pub enum AccountStatus {
    #[sea_orm(string_value = "Created")]
    #[default]
    Created,
    #[sea_orm(string_value = "Enabled")]
    Enabled, // If in use, cannot delete
    #[sea_orm(string_value = "Frozen")]
    Frozen,
    #[sea_orm(string_value = "Blocked")]
    Blocked,
    #[sea_orm(string_value = "Disabled")]
    Disabled,
    #[sea_orm(string_value = "Deleted")]
    Deleted, // Soft deletes
}

#[derive(Default, Debug, Clone, PartialEq, Eq, EnumString, EnumIter, DeriveActiveEnum, Deserialize, Serialize)]
#[sea_orm(rs_type = "String", db_type = "String(Some(1))")]
pub enum AccountType {
    #[sea_orm(string_value = "ACC001")]
    #[default]
    ACC001,
    #[sea_orm(string_value = "ACC002")]
    ACC002,
    #[sea_orm(string_value = "ACC003")]
    ACC003,
    #[sea_orm(string_value = "ACC004")]
    ACC004,
    #[sea_orm(string_value = "ACC005")]
    ACC005,
    #[sea_orm(string_value = "ACC006")]
    ACC006,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, EnumString, EnumIter, DeriveActiveEnum, Deserialize, Serialize)]
#[sea_orm(rs_type = "String", db_type = "String(Some(1))")]
pub enum TransferStatus {
    #[sea_orm(string_value = "Apply")]
    #[default]
    Apply,
    #[sea_orm(string_value = "Pending")]
    Pending,
    #[sea_orm(string_value = "Completed")]
    Completed,
    #[sea_orm(string_value = "Failure")]
    Failure,
    #[sea_orm(string_value = "Handled")]
    Handled,
    #[sea_orm(string_value = "Canceled")]
    Canceled,
    #[sea_orm(string_value = "Ignored")]
    Ignored,
}

#[derive(Default, Copy, Debug, Clone, PartialEq, Eq, EnumString, EnumIter, DeriveActiveEnum, Deserialize, Serialize)]
#[sea_orm(rs_type = "String", db_type = "String(Some(1))")]
pub enum CurrencyType {
    #[default]
    #[sea_orm(string_value = "USD")]
    USD,
    #[sea_orm(string_value = "EUR")]
    EUR,
    #[sea_orm(string_value = "CNY")]
    CNY,
    #[sea_orm(string_value = "HKD")]
    HKD,
    #[sea_orm(string_value = "SGD")]
    SGD,
    #[sea_orm(string_value = "USDT")]
    USDT,
    #[sea_orm(string_value = "USDC")]
    USDC,
    #[sea_orm(string_value = "BTC")]
    BTC,
    #[sea_orm(string_value = "ETH")]
    ETH,
    #[sea_orm(string_value = "SOL")]
    SOL,
    #[sea_orm(string_value = "ATOM")]
    ATOM,
    #[sea_orm(string_value = "DOGE")]
    DOGE,
    #[sea_orm(string_value = "BNB")]
    BNB,
    #[sea_orm(string_value = "GPT")]
    GPT,
    #[sea_orm(string_value = "XRP")]
    XRP,
    #[sea_orm(string_value = "FIL")]
    FIL,
    #[sea_orm(string_value = "DOT")]
    DOT,
}