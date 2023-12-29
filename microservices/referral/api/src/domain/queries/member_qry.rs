use tonic::Status;
use uuid::Uuid;
use shared::StatusUtil;
use crate::domain::entities::member;
use crate::infra::repositories::member_query::MemberDbQuery;

pub struct MemberQuery;

impl MemberQuery {
    pub async fn get_member_by_id(user_id: Uuid) -> Result<Option<member::Model>, Status> {
        MemberDbQuery::get_member_by_id(user_id).await
            .map_err(|e| StatusUtil::neo4j_error(e))
    }

    pub async fn get_my_referral(user_id: Uuid) -> Result<Option<member::Model>, Status> {
        MemberDbQuery::get_referral_member(user_id).await
            .map_err(|e| StatusUtil::neo4j_error(e))
    }

    pub async fn get_my_referees(user_id: Uuid) -> Result<Vec<member::Model>, Status> {
        MemberDbQuery::get_my_referees(user_id).await
            .map_err(|e| StatusUtil::neo4j_error(e))
    }
}