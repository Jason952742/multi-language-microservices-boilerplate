use tonic::Status;
use uuid::Uuid;
use shared::GrpcStatusTool;
use crate::domain::entities::member;
use crate::infra::repositories::member_query::MemberDbQuery;

pub struct MemberQuery;

impl MemberQuery {
    pub async fn get_member_by_id(user_id: Uuid) -> Result<Option<member::Model>, Status> {
        MemberDbQuery::get_member_by_id(user_id).await
            .map_err(|e| GrpcStatusTool::neo4j_error(e))
    }

    pub async fn get_member_by_code(code: &str) -> Result<Option<member::Model>, Status> {
        MemberDbQuery::get_member_by_code(code).await
            .map_err(|e| GrpcStatusTool::neo4j_error(e))
    }

    pub async fn get_referrer(user_id: Uuid) -> Result<Option<member::Model>, Status> {
        MemberDbQuery::get_referrer(user_id).await
            .map_err(|e| GrpcStatusTool::neo4j_error(e))
    }

    pub async fn get_referrals(user_id: Uuid) -> Result<Vec<member::Model>, Status> {
        MemberDbQuery::get_referrals(user_id).await
            .map_err(|e| GrpcStatusTool::neo4j_error(e))
    }
}