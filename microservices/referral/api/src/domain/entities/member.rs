use chrono::{DateTime, Utc};
use uuid::Uuid;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Model {
    pub member_id: Uuid,
    pub user_id: Uuid,
    pub user_name: String,
    pub referral_code: String,
    pub hierarchy: i32,
    pub description: String,
    pub created_at: DateTime<Utc>,
    #[serde(skip_deserializing)]
    pub deleted: bool,
}
