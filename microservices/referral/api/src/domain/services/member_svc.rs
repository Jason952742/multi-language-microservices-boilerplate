use chrono::{Utc};
use tonic::Status;
use uuid::Uuid;
use shared::utils::GrpcStatusTool;
use crate::domain::commands::member_cmd::MemberEvent;
use crate::domain::entities::member;
use crate::domain::messages::{MemberReferralMsg};
use crate::infra::repositories::member_mutation::MemberDbMutation;
use crate::infra::repositories::member_query::MemberDbQuery;

pub struct MemberService;

impl MemberService {
    pub async fn create_referral(user_id: Uuid, event: MemberReferralMsg) -> Result<MemberEvent, Status> {
        match MemberDbQuery::check_member(user_id).await.map_err(|e| GrpcStatusTool::neo4j_error(e))? {
            true => Err(Status::already_exists("member already exists")),
            false => {
                let referrer = match event.referrer_id {
                    None => None,
                    Some(id) => {
                        MemberDbQuery::get_member_by_id(id).await.map_err(|e| GrpcStatusTool::neo4j_error(e))?
                    }
                };

                let form_data: member::Model = member::Model {
                    member_id: event.member_id,
                    user_id,
                    user_name: event.user_name,
                    referral_code: event.referral_code,
                    hierarchy: if referrer.as_ref().is_some() { referrer.as_ref().unwrap().hierarchy + 1 } else { 0 },
                    created_at: Utc::now(),
                    ..Default::default()
                };

                match MemberDbMutation::create_member(form_data).await {
                    Ok(_) => {
                        if let Some(r) = referrer {
                            let _ = MemberDbMutation::create_relationship(user_id, r.user_id).await
                                .map_err(|e| GrpcStatusTool::neo4j_error(e));
                        }
                        Ok(MemberEvent::Created)
                    }
                    Err(_) => Err(Status::internal("Failed to create"))
                }
            }
        }
    }

    pub async fn update_referral(user_id: Uuid, description: String) -> Result<MemberEvent, Status> {
        match MemberDbQuery::get_member_by_id(user_id).await {
            Ok(opt) => match opt {
                None => Err(Status::not_found("member not found")),
                Some(_) => {
                    let _ = MemberDbMutation::update_member(user_id, &description).await;
                    Ok(MemberEvent::Updated)
                }
            }
            Err(e) => Err(Status::internal(e.to_string()))
        }
    }

    pub async fn bind_referral(user_id: Uuid, referrer_id: Uuid) -> Result<MemberEvent, Status> {
        let res = MemberDbMutation::create_relationship(user_id, referrer_id).await.map_err(|e| GrpcStatusTool::neo4j_error(e));
        match res {
            Ok(_) => Ok(MemberEvent::Bound),
            Err(e) => Err(e)
        }
    }
}
