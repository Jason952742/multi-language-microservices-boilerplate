use scylla::batch::{Batch};
use shared::scylla::transport::errors::QueryError;
use shared::scylladb::ScyllaPool;
use crate::domain::aggregates::account_ar::Account;
use crate::domain::aggregates::member_ar::Member;
use crate::domain::aggregates::referral_ar::Referral;
use crate::domain::entities::{eventsource};
use crate::domain::entities::enums::AggregateType;

pub struct EventSourceDbMutation;

impl EventSourceDbMutation {
    pub async fn create_eventsource(table: &str, event: eventsource::Model) -> Result<(), QueryError> {
        let session = ScyllaPool::connection().await;

        session.query(format!("INSERT INTO eventflow.{} (id, txn_id, aggregate_id, aggregate_type, sequence, event_type, event_version, payload, metadata, created_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)", table), event).await?;

        Ok(())
    }

    pub async fn batch_eventsource(events: Vec<&eventsource::Model>) -> Result<(), QueryError> {
        let session = ScyllaPool::connection().await;
        let mut batch: Batch = Default::default();
        let mut batch_values = Vec::new();

        events.into_iter().for_each(|evt| {
            let table = match evt.aggregate_type {
                AggregateType::Member => Member::TABLE_NAME,
                AggregateType::Account => Account::TABLE_NAME,
                AggregateType::Referral => Referral::TABLE_NAME,
            };

            batch.append_statement(format!("INSERT INTO eventflow.{} (id, txn_id, aggregate_id, aggregate_type, sequence, event_type, event_version, payload, metadata, created_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)", table).as_str());
             batch_values.push(evt);
        });

        session.batch(&batch, batch_values).await?;

        Ok(())
    }
}

#[tokio::test]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    use rust_decimal_macros::dec;
    use crate::domain::entities::enums::{AggregateType};
    use crate::domain::aggregates::account_ar::{Account, AccountEvent};

    use serde_json::Value;
    use std::str::FromStr;
    use chrono::{Utc};
    use uuid::Uuid;

    let id = Uuid::from_str("ae053855-9321-404c-a3ce-b57e155487cf").unwrap();
    let user_id = Uuid::new_v4();

    let event = AccountEvent::CustomerDepositedMoney {
        amount: dec!(1000.0),
        balance: dec!(2000.0),
    };
    let payload: Value = event.clone().into();

    let tr = eventsource::Model {
        id: Uuid::new_v4(),
        txn_id: None,
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