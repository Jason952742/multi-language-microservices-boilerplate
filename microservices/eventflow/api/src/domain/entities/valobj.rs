use serde_derive::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::NaiveDateTime;
use rust_decimal::Decimal;
use crate::domain::entities::enums::{CurrencyType, MemberType, PaymentType};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct User {
    pub user_id: Uuid,
    pub user_name: String,
    pub member_id: Uuid,
    pub member_type: MemberType,
    pub subscription_end_date: NaiveDateTime,
    pub account_id: Uuid,
    pub account_balance: Decimal,
    pub refer_code: String,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Payment {
    pub payment_type: PaymentType,
    pub currency_type: CurrencyType,
    pub amount: Decimal,
    pub paid_at: NaiveDateTime,
    pub receipt: String,
    pub equipment_id: String
}
