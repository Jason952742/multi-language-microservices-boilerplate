
use chrono::format::Numeric::*;
use chrono::{DateTime, Utc};
use scylla::_macro_internal::{CqlValue, FromCqlVal, SerializeCql};
use scylla::{FromRow, SerializeRow};
use uuid::Uuid;
use serde::{Deserialize, Serialize};
use crate::domain::entities::enums::{AggregateType};


#[derive(Default, Clone, Debug, PartialEq, Eq, Deserialize, Serialize, FromRow, SerializeRow, SerializeCql)]
pub struct Model {
    pub aggregate_id: Uuid,
    pub aggregate_type: AggregateType,
    pub sequence: i64,
    pub event_type: String,
    pub event_version: String,
    pub payload: String,
    pub metadata: String,
    pub created_at: DateTime<Utc>
}
