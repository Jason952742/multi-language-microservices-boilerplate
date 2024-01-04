use tonic::Status;
use uuid::Uuid;
use shared::GrpcStatusTool;
use crate::domain::entities::transaction;
use crate::infra::repositories::transaction_query::TransactionDbQuery;

pub struct MemberQuery;

impl MemberQuery {
    // pub async fn get_member_by_id(user_id: Uuid) -> Result<Option<transaction::Model>, Status> {
    //     MemberDbQuery::get_member_by_id(user_id).await
    //         .map_err(|e| GrpcStatusTool::neo4j_error(e))
    // }
    //
    // pub async fn get_my_referral(user_id: Uuid) -> Result<Option<transaction::Model>, Status> {
    //     MemberDbQuery::get_referral_member(user_id).await
    //         .map_err(|e| GrpcStatusTool::neo4j_error(e))
    // }
    //
    // pub async fn get_my_referees(user_id: Uuid) -> Result<Vec<transaction::Model>, Status> {
    //     MemberDbQuery::get_my_referees(user_id).await
    //         .map_err(|e| GrpcStatusTool::neo4j_error(e))
    // }
}