use serde::{Deserialize, Serialize};
use serde_json::json;
use strum_macros;
use strum_macros::{EnumIter, EnumString};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, EnumString, EnumIter, strum_macros::Display)]
pub enum MemberType {
    Wood,
    Iron,
    Brass,
    Silver,
    Gold,
    Platinum,
    Diamond,
    Sphene,
}

impl Default for MemberType {
    fn default() -> Self {
        MemberType::Wood
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemberCreatedEvent {
    pub user_id: Uuid,
    pub user_name: String,
    pub member_type: MemberType,
    pub member_id: Uuid,
    pub login_creds: String,
    pub level: i32,
    pub my_referrer_code: String,
    pub referee_code: String,
}

impl Into<Vec<u8>> for MemberCreatedEvent {
    fn into(self) -> Vec<u8> {
        serde_json::to_vec(&json!(self)).expect("Error decoding payload")
    }
}

impl From<&[u8]> for MemberCreatedEvent {
    fn from(v: &[u8]) -> Self {
        serde_json::from_slice::<MemberCreatedEvent>(v).unwrap()
    }
}
