use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use serde_json::json;
use strum_macros;
use uuid::Uuid;
use crate::domain::entities::enums::{CurrencyType, TransferType};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountCreated {
    pub user_id: Uuid,
    pub account_id: Uuid
}

impl Into<Vec<u8>> for AccountCreated {
    fn into(self) -> Vec<u8> {
        serde_json::to_vec(&json!(self)).expect("Error decoding payload")
    }
}

impl From<&[u8]> for AccountCreated {
    fn from(v: &[u8]) -> Self {
        serde_json::from_slice::<AccountCreated>(v).unwrap()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountTransaction {
    pub user_id: Uuid,
    pub account_id: Uuid,
    pub currency_type: CurrencyType,
    pub amount: Decimal,
    pub balance: Decimal,
    pub transfer_type: TransferType,
}

impl Into<Vec<u8>> for AccountTransaction {
    fn into(self) -> Vec<u8> {
        serde_json::to_vec(&json!(self)).expect("Error decoding payload")
    }
}

impl From<&[u8]> for AccountTransaction {
    fn from(v: &[u8]) -> Self {
        serde_json::from_slice::<AccountTransaction>(v).unwrap()
    }
}
