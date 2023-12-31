use chrono::{DateTime, Utc};
use scylla::{FromRow, SerializeRow, SerializeCql};
use uuid::Uuid;
use serde::{Deserialize, Serialize};
use crate::domain::entities::enums::{AggregateType};


#[derive(Default, Clone, Debug, PartialEq, Eq, Deserialize, Serialize, FromRow, SerializeRow, SerializeCql)]
pub struct Model {
    pub id: Uuid,
    pub txn_id: Option<Uuid>,
    pub aggregate_id: Uuid,
    pub aggregate_type: AggregateType,
    pub sequence: i64,
    pub event_type: String,
    pub event_version: String,
    pub payload: String,
    pub metadata: String,
    pub created_at: DateTime<Utc>
}
