use chrono::NaiveDateTime;
use uuid::Uuid;
use serde::{Deserialize, Serialize};
use crate::domain::messages::MemberType;

#[derive(Clone, Default, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Model {
    pub user_id: Uuid,
    pub user_name: String,
    pub member_type: MemberType,
    pub member_id: Uuid,
    pub login_creds: String,
    pub level: i32,
    pub my_referrer_code: String,
    pub referee_code: String,

    pub hierarchy: i32,
    pub active: bool,
    pub description: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub enabled: bool,
    pub version: i32,
    #[serde(skip_deserializing)]
    pub deleted: bool,
}
