use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemberCreatedMsg {
    pub user_id: Uuid,
    pub user_name: String,
    pub member_id: Uuid,
    pub sub_end_date: DateTime<Utc>
}

impl Into<Vec<u8>> for MemberCreatedMsg {
    fn into(self) -> Vec<u8> {
        serde_json::to_vec(&json!(self)).expect("Error decoding payload")
    }
}

impl From<&[u8]> for MemberCreatedMsg {
    fn from(v: &[u8]) -> Self {
        serde_json::from_slice::<MemberCreatedMsg>(v).unwrap()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemberSubscribedMsg {
    pub user_id: Uuid,
    pub member_id: Uuid,
    pub sub_end_date: DateTime<Utc>
}

impl Into<Vec<u8>> for MemberSubscribedMsg {
    fn into(self) -> Vec<u8> {
        serde_json::to_vec(&json!(self)).expect("Error decoding payload")
    }
}

impl From<&[u8]> for MemberSubscribedMsg {
    fn from(v: &[u8]) -> Self {
        serde_json::from_slice::<MemberSubscribedMsg>(v).unwrap()
    }
}
