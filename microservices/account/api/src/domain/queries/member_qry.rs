use tonic::Status;
use uuid::Uuid;
use shared::utils::GrpcStatusTool;
use crate::domain::entities::enums::{MemberStatus, MemberType};
use crate::domain::entities::member;
use crate::infra::repositories::member_query::MemberOrmQuery;

pub struct MemberQuery;

impl MemberQuery {

    pub async fn get_members(page: u64, per_page: u64, status: Option<MemberStatus>, member_type: Option<MemberType>, level: Option<i32>) -> Result<(Vec<member::Model>, u64), Status> {
        MemberOrmQuery::find_members_in_page(page, per_page, status, member_type, level).await
            .map_err(|e| GrpcStatusTool::db_error(e))
    }

    pub async fn get_member_by_user_id(user_id: Uuid) -> Result<Option<member::Model>, Status> {
        MemberOrmQuery::_get_member_by_user_id(user_id).await
            .map_err(|e| GrpcStatusTool::db_error(e))
    }

    pub async fn get_member_by_id(id: Uuid) -> Result<Option<member::Model>, Status> {
        MemberOrmQuery::get_member_by_id(id).await
            .map_err(|e| GrpcStatusTool::db_error(e))
    }

}
