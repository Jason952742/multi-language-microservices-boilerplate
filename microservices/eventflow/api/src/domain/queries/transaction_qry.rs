use tonic::Status;
use uuid::Uuid;
use shared::utils::GrpcStatusTool;
use crate::domain::entities::enums::TransactionType;
use crate::domain::entities::transaction;
use crate::infra::repositories::transaction_query::TransactionDbQuery;

pub struct TransactionQuery;

impl TransactionQuery {
    pub async fn get_transaction_by_id(id: Uuid) -> Result<Option<transaction::Model>, Status> {
        TransactionDbQuery::get_transaction_by_id(id).await
            .map_err(|e| GrpcStatusTool::invalid(e.to_string().as_str()))
    }

    pub async fn get_transactions(user_id: Uuid, transaction_type: TransactionType) -> Result<Vec<transaction::Model>, Status> {
        TransactionDbQuery::get_transactions(user_id, transaction_type).await
            .map_err(|e| GrpcStatusTool::invalid(e.to_string().as_str()))
    }

}