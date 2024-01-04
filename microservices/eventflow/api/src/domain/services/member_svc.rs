use chrono::Local;
use tonic::Status;
use uuid::Uuid;
use shared::GrpcStatusTool;
use crate::domain::commands::eventflow_cmd::EventflowEvent;
use crate::domain::entities::member;
use crate::domain::messages::{MemberCreatedEvent, MemberType};
use crate::infra::repositories::member_mutation::MemberDbMutation;
use crate::infra::repositories::member_query::MemberDbQuery;

pub struct MemberService;

impl MemberService {
    pub async fn create_referral(user_id: Uuid, event: MemberCreatedEvent) -> Result<EventflowEvent, Status> {
        // match MemberDbQuery::check_member(user_id).await.map_err(|e| GrpcStatusTool::neo4j_error(e))? {
        //     true => Err(Status::already_exists("member already exists")),
        //     false => {
        //         let referrer: Option<member::Model> = if event.referee_code != "system" {
        //             MemberDbQuery::get_member_by_my_referrer_code(&event.referee_code).await.map_err(|e| GrpcStatusTool::neo4j_error(e))?
        //         } else { None };
        //
        //         let form_data: member::Model = member::Model {
        //             user_id,
        //             user_name: event.user_name,
        //             member_type: event.member_type,
        //             member_id: event.member_id,
        //             login_creds: event.login_creds,
        //             level: event.level,
        //             my_referrer_code: event.my_referrer_code,
        //             referee_code: event.referee_code,
        //             hierarchy: if referrer.as_ref().is_some() { referrer.as_ref().unwrap().hierarchy + 1 } else { 0 },
        //             active: true,
        //             created_at: Local::now().naive_local(),
        //             updated_at: Local::now().naive_local(),
        //             ..Default::default()
        //         };
        //
        //         match MemberDbMutation::create_member(form_data).await {
        //             Ok(_) => {
        //                 if let Some(r) = referrer {
        //                     let _ = MemberDbMutation::create_relationship(user_id, r.user_id).await
        //                         .map_err(|e| GrpcStatusTool::neo4j_error(e));
        //                 }
        //                 Ok(EventflowEvent::Created)
        //             }
        //             Err(_) => Err(Status::internal("Failed to create"))
        //         }
        //     }
        // }
        todo!()
    }

    pub async fn update_referral(user_id: Uuid, member_type: MemberType, level: i32, active: bool, description: String) -> Result<EventflowEvent, Status> {
        // match MemberDbQuery::get_member_by_id(user_id).await {
        //     Ok(opt) => match opt {
        //         None => Err(Status::not_found("member not found")),
        //         Some(m) => {
        //             let _ = MemberDbMutation::update_member(member::Model {
        //                 member_type,
        //                 level,
        //                 active,
        //                 description,
        //                 updated_at: Local::now().naive_local(),
        //                 version: m.version + 1,
        //                 ..m
        //             }).await;
        //             Ok(EventflowEvent::Updated)
        //         }
        //     }
        //     Err(e) => Err(Status::internal(e.to_string()))
        // }
        todo!()
    }

    pub async fn bind_referral(user_id: Uuid, referral_id: Uuid) -> Result<EventflowEvent, Status> {
        // let res = MemberDbMutation::create_relationship(user_id, referral_id).await.map_err(|e| GrpcStatusTool::neo4j_error(e));
        // match res {
        //     Ok(_) => Ok(EventflowEvent::Bound),
        //     Err(e) => Err(e)
        // }
        todo!()
    }
}
