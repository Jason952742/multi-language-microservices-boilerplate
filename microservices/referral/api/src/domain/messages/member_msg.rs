use serde::{Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemberReferralEvent {
    pub user_id: Uuid,
    pub user_name: String,
    pub member_id: Uuid,
    pub referral_code: String,
    pub referrer_id: Option<Uuid>,
}

impl Into<Vec<u8>> for MemberReferralEvent {
    fn into(self) -> Vec<u8> {
        serde_json::to_vec(&json!(self)).expect("Error decoding payload")
    }
}

impl From<&[u8]> for MemberReferralEvent {
    fn from(v: &[u8]) -> Self {
        serde_json::from_slice::<MemberReferralEvent>(v).unwrap()
    }
}
