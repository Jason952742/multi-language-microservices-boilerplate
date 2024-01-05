
use chrono::format::Numeric::*;
use chrono::{DateTime, Utc};
use scylla::{FromRow, SerializeCql, SerializeRow};
use uuid::Uuid;
use serde::{Deserialize, Serialize};
use crate::domain::entities::enums::{TransactionStatus, TransactionType};


#[derive(Default, Clone, Debug, PartialEq, Eq, Deserialize, Serialize, FromRow, SerializeRow, SerializeCql)]
pub struct Model {
    pub id: Uuid,
    pub transaction_type: TransactionType,
    pub status: TransactionStatus,
    pub user_id: Uuid,
    pub data: String,
    pub event_ids: Option<String>,
    pub rollback_id: Option<Uuid>,
    pub description: String,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub enabled: bool,
    pub version: i32,
    #[serde(skip_deserializing)]
    pub deleted: bool,
    #[serde(skip_deserializing)]
    pub deleted_at: Option<DateTime<Utc>>,
}
