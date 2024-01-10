use std::cmp::Ordering;
use futures::{StreamExt};
use scylla::IntoTypedRows;
use uuid::Uuid;
use shared::datasource::scylladb::ScyllaPool;
use crate::domain::entities::enums::TransactionType;
use crate::domain::entities::transaction;

pub struct TransactionDbQuery;

impl TransactionDbQuery {
    pub async fn get_transaction_by_id(id: Uuid) -> Result<Option<transaction::Model>, Box<dyn std::error::Error>> {
        let session = ScyllaPool::connection().await;

        let query = session.query("SELECT id, transaction_type, status, user_id, payload, events, rollback_id, description, created_at, updated_at, enabled, version, deleted, deleted_at FROM eventflow.transactions WHERE id = ?", (id, )).await?;

        if let Some(rows) = query.rows {
            println!("{:?}", rows);
            for row_data in rows.into_typed::<transaction::Model>() {
                let row_data = row_data?;
                return Ok(Some(row_data));
            }
        };

        Ok(None)
    }

    pub async fn get_transactions(user_id: Uuid, transaction_type: TransactionType) -> Result<Vec<transaction::Model>, Box<dyn std::error::Error>> {

        fn compare_models(a: &transaction::Model, b: &transaction::Model) -> Ordering {
            a.created_at.cmp(&b.created_at)
        }

        let session = ScyllaPool::connection().await;
        let mut list: Vec<transaction::Model> = vec![];

        let mut rows_stream = session.query_iter("SELECT id, transaction_type, status, user_id, payload, events, rollback_id, description, created_at, updated_at, enabled, version, deleted, deleted_at FROM eventflow.transactions WHERE user_id = ? AND transaction_type = ? ALLOW FILTERING", (user_id, transaction_type)).await?.into_typed::<transaction::Model>();

        while let Some(next_row_res) = rows_stream.next().await {
            list.push(next_row_res.unwrap());
        }

        list.sort_by(compare_models);

        Ok(list)
    }
}

#[tokio::test]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    use std::str::FromStr;

    // let id = Uuid::from_str("2e4c38c6-133c-43c1-a6ad-fa91dee3a06b").unwrap();
    // let r = TransactionDbQuery::get_transaction_by_id(id).await?;
    // println!("{:?}", r);

    let user_id = Uuid::from_str("7aa9bba7-18c2-429e-8f20-3c25bf3d6e15").unwrap();
    let transaction_type = TransactionType::UserCreate;

    let res1 = TransactionDbQuery::get_transactions(user_id, transaction_type).await?;
    println!("Paging state: {:?} (rows)", res1);

    Ok(())
}