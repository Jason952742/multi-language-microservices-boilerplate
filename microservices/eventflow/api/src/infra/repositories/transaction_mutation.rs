use uuid::Uuid;
use shared::scylla::transport::errors::QueryError;
use shared::scylladb::ScyllaPool;
use crate::domain::entities::enums::{TransactionStatus};
use crate::domain::entities::transaction;

pub struct TransactionDbMutation;

impl TransactionDbMutation {
    pub async fn create_transaction(form_data: transaction::Model) -> Result<(), QueryError> {
        let session = ScyllaPool::connection().await;

        session.query(
            "INSERT INTO eventflow.transaction (id, transaction_type, status, user_id, payload, events, rollback_id, description, created_at, updated_at, enabled, version, deleted, deleted_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)", form_data).await?;

        Ok(())
    }

    pub async fn update_transaction(id: Uuid, status: TransactionStatus, events: Vec<String>, rollback_id: Option<Uuid>) -> Result<(), QueryError> {
        let session = ScyllaPool::connection().await;
        let event_ids = events.join(",");

        session.query(
            "UPDATE eventflow.transaction SET status = ?, events = ?, rollback_id = ? WHERE id = ?", (
                status, event_ids, rollback_id, id,
            )).await?;

        Ok(())
    }
}

#[tokio::test]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    Ok(())
}