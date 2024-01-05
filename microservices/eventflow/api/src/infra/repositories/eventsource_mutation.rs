use std::str::FromStr;
use chrono::{Utc};
use uuid::Uuid;
use shared::scylla::transport::errors::QueryError;
use shared::scylladb::ScyllaPool;
use crate::domain::aggregates::account_ar::{Account, AccountEvent};
use crate::domain::entities::{eventsource};
use serde_json::Value;
use crate::domain::entities::enums::{AggregateType, CurrencyType};

pub struct EventSourceDbMutation;

impl EventSourceDbMutation {
    pub async fn create_eventsource(table: &str, form_data: eventsource::Model) -> Result<(), QueryError> {
        let session = ScyllaPool::connection().await;

        session.query(format!("INSERT INTO eventflow.{} (aggregate_id, aggregate_type, sequence, event_type, event_version, payload, metadata, created_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?)", table), form_data).await?;

        Ok(())
    }
}

#[tokio::test]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let id = Uuid::from_str("ae053855-9321-404c-a3ce-b57e155487cf").unwrap();
    let user_id = Uuid::new_v4();

    let event = AccountEvent::CustomerDepositedMoney {
        amount: 1000.0,
        balance: 2000.0,
    };
    let payload: Value = event.clone().into();

    let tr = eventsource::Model {
        aggregate_id: id,
        aggregate_type: AggregateType::Account,
        sequence: Utc::now().timestamp(),
        event_type: event.event_type(),
        event_version: event.event_version(),
        payload: payload.to_string(),
        metadata: "".to_string(),
        created_at: Utc::now(),
    };

    println!("{:?}", tr);

    EventSourceDbMutation::create_eventsource(Account::TABLE_NAME, tr).await?;


    Ok(())
}