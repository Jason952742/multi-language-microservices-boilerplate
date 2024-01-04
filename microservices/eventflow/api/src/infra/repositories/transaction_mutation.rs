use chrono::Utc;
use uuid::Uuid;
use shared::scylla::transport::errors::QueryError;
use shared::scylladb::ScyllaPool;
use crate::domain::entities::enums::{TransactionStatus, TransactionType};
use crate::domain::entities::transaction;

pub struct TransactionDbMutation;

impl TransactionDbMutation {
    pub async fn create_transaction(form_data: transaction::Model) -> Result<(), QueryError> {
        let session = ScyllaPool::connection().await;

        session.query(
            "INSERT INTO eventflow.transaction (id, transaction_type, status, user_id, data, event_ids, rollback_id, description, created_at, updated_at, enabled, version, deleted, deleted_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)", form_data).await?;

        Ok(())
    }

    pub async fn update_transaction(id: Uuid, status: TransactionStatus, event_ids: Vec<Uuid>, rollback_id: Option<Uuid>) -> Result<(), QueryError> {
        let session = ScyllaPool::connection().await;
        let status = status.to_string();
        let event_ids = event_ids.iter()
            .map(|uuid| uuid.to_string())
            .collect::<Vec<String>>()
            .join(",");

        session.query(
            "UPDATE eventflow.transaction SET status = ?, event_ids = ?, rollback_id = ? WHERE id = ?", (
                status, event_ids, rollback_id, id,
            )).await?;

        Ok(())
    }
}

#[tokio::test]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let id = Uuid::new_v4();

    let tr = transaction::Model {
        id: id.clone(),
        transaction_type: TransactionType::UserCreate.to_string(),
        status: TransactionStatus::Apply.to_string(),
        user_id: Uuid::new_v4(),
        data: "bbb".to_string(),
        created_at: Utc::now(),
        updated_at: Utc::now(),
        ..Default::default()
    };

    TransactionDbMutation::create_transaction(tr).await?;

    let event_ids = vec![
      Uuid::new_v4(), Uuid::new_v4(), Uuid::new_v4()
    ];
    TransactionDbMutation::update_transaction(id, TransactionStatus::Completed, event_ids, None).await?;


    Ok(())
}