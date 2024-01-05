use serde::{Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemberCreated {
    pub user_id: Uuid,
    pub user_name: String,
    pub member_id: Uuid
}

impl Into<Vec<u8>> for MemberCreated {
    fn into(self) -> Vec<u8> {
        serde_json::to_vec(&json!(self)).expect("Error decoding payload")
    }
}

impl From<&[u8]> for MemberCreated {
    fn from(v: &[u8]) -> Self {
        serde_json::from_slice::<MemberCreated>(v).unwrap()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemberSubscription {
    pub user_id: Uuid,
    pub member_id: Uuid,
    pub duration: i32
}

impl Into<Vec<u8>> for MemberSubscription {
    fn into(self) -> Vec<u8> {
        serde_json::to_vec(&json!(self)).expect("Error decoding payload")
    }
}

impl From<&[u8]> for MemberSubscription {
    fn from(v: &[u8]) -> Self {
        serde_json::from_slice::<MemberSubscription>(v).unwrap()
    }
}
