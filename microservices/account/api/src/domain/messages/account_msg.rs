use crate::domain::entities::enums::CurrencyType;
use serde::{Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountCreatedMsg {
  pub user_id: Uuid,
  pub account_id: Uuid,
  pub ccy_type: CurrencyType,
}

impl Into<Vec<u8>> for AccountCreatedMsg {
  fn into(self) -> Vec<u8> {
    serde_json::to_vec(&json!(self)).expect("Error decoding payload")
  }
}

impl From<&[u8]> for AccountCreatedMsg {
  fn from(v: &[u8]) -> Self {
    serde_json::from_slice::<AccountCreatedMsg>(v).unwrap()
  }
}
