use std::str::FromStr;
use scylla::cql_to_rust::FromRowError;
use scylla::IntoTypedRows;
use scylla::transport::errors::QueryError;
use uuid::Uuid;
use shared::{convert_to_bool, convert_to_i32, opt_to_uuid, string_to_datetime};
use shared::scylladb::ScyllaPool;
use crate::domain::entities::transaction;

pub struct TransactionDbQuery;

impl TransactionDbQuery {
    pub async fn get_transaction_by_id(id: Uuid) -> Result<Option<transaction::Model>, Box<dyn std::error::Error>> {
        let session = ScyllaPool::connection().await;

        let query = session.query("SELECT id, transaction_type, status, user_id, data, event_ids, rollback_id, description, created_at, updated_at, enabled, version, deleted, deleted_at FROM eventflow.transaction WHERE id = ?", (id, )).await?;

        if let Some(rows) = query.rows {
            println!("{:?}", rows);
            for row_data in rows.into_typed::<transaction::Model>() {
                let row_data = row_data?;
                return Ok(Some(row_data));
            }
        };

        Ok(None)
    }

    pub async fn get_transactions() -> Result<Vec<transaction::Model>, QueryError> {
        let session = ScyllaPool::connection().await;


        Ok(vec![])
    }
}

#[tokio::test]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let id = Uuid::from_str("9da2e969-c313-4f89-a2f5-d24786e899f4").unwrap();

    let r = TransactionDbQuery::get_transaction_by_id(id).await?;

    println!("{:?}", r);

    Ok(())
}