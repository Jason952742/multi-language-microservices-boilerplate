use chrono::{DateTime, Utc};
use tonic::Status;
use uuid::Uuid;
use shared::utils::GrpcStatusTool;
use crate::domain::commands::member_cmd::MemberEvent;
use crate::domain::entities::enums::MemberType;
use crate::domain::entities::member;
use crate::infra::repositories::member_mutation::MemberOrmMutation;
use crate::infra::repositories::member_query::{MemberOrmQuery};

pub struct MemberService;

impl MemberService {
    pub async fn create_member(id: Uuid, user_id: Uuid, user_name: String, sub_end_date: DateTime<Utc>) -> Result<MemberEvent, Status> {
        match MemberOrmQuery::get_member_by_id(id).await.map_err(|e| GrpcStatusTool::db_error(e))? {
            Some(_) => Err(Status::already_exists("member already exists")),
            None => {
                let form_data: member::Model = member::Model { id, user_id, user_name, sub_end_date, ..Default::default() };
                match MemberOrmMutation::create_member(form_data).await {
                    Ok(id) => Ok(MemberEvent::Created { id }),
                    Err(_) => Err(Status::internal("Failed to create"))
                }
            }
        }
    }

    pub async fn subscribe(id: Uuid, user_id: Uuid, sub_end_date: DateTime<Utc>) -> Result<MemberEvent, Status> {
        match MemberOrmQuery::get_member_by_id(id).await.map_err(|e| GrpcStatusTool::db_error(e))? {
            Some(member) => {
                let _ = MemberOrmMutation::subscribe(user_id, sub_end_date).await.map_err(|e| GrpcStatusTool::db_error(e))?;
                Ok(MemberEvent::Subscribed)
            }
            None => Err(Status::not_found("member not found"))
        }
    }

    pub async fn update_member(user_id: Uuid, member_type: MemberType, level: i32, active: bool, description: String) -> Result<MemberEvent, Status> {
        match MemberOrmMutation::update_member(user_id, member_type, level, active, description).await {
            Ok(_) => Ok(MemberEvent::Updated),
            Err(e) => Err(GrpcStatusTool::db_error(e))
        }
    }

    pub async fn disabled_member(user_id: Uuid) -> Result<MemberEvent, Status> {
        match MemberOrmMutation::disable_member(user_id).await {
            Ok(_) => Ok(MemberEvent::Disabled),
            Err(e) => Err(GrpcStatusTool::db_error(e))
        }
    }
}
