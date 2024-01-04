use serde::{Deserialize, Serialize};
use serde_json::json;
use strum_macros;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemberReferral {
    pub user_id: Uuid,
    pub user_name: String,
    pub member_id: i32,
    pub refer_code: String,
    pub referee_code: String,
}

impl Into<Vec<u8>> for MemberReferral {
    fn into(self) -> Vec<u8> {
        serde_json::to_vec(&json!(self)).expect("Error decoding payload")
    }
}

impl From<&[u8]> for MemberReferral {
    fn from(v: &[u8]) -> Self {
        serde_json::from_slice::<MemberReferral>(v).unwrap()
    }
}
