use futures::{StreamExt};
use uuid::Uuid;
use shared::datasource::scylladb::ScyllaPool;
use crate::domain::entities::{eventsource};

pub struct EventSourceDbQuery;

impl EventSourceDbQuery {

    pub async fn get_eventsources(table: &str, aggregate_id: Uuid) -> Result<Vec<eventsource::Model>, Box<dyn std::error::Error>> {

        let session = ScyllaPool::connection().await;
        let mut list: Vec<eventsource::Model> = vec![];

        let mut rows_stream = session.query_iter(format!("SELECT id, txn_id, aggregate_id, aggregate_type, sequence, event_type, event_version, payload, metadata, created_at FROM eventflow.{} WHERE aggregate_id = ? ORDER BY sequence ASC ALLOW FILTERING", table), (aggregate_id, )).await?.into_typed::<eventsource::Model>();

        while let Some(next_row_res) = rows_stream.next().await {
            list.push(next_row_res.unwrap());
        }

        Ok(list)
    }
}

#[tokio::test]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    use std::str::FromStr;
    use crate::domain::aggregates::account_ar::Account;

    let id = Uuid::from_str("ae053855-9321-404c-a3ce-b57e155487cf").unwrap();

    let res1 = EventSourceDbQuery::get_eventsources(Account::TABLE_NAME, id).await?;
    println!("Paging state: {:?} (rows)", res1);

    Ok(())
}