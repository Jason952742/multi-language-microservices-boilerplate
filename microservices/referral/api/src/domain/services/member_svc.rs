use std::error::Error;
use chrono::Local;
use tonic::Status;
use uuid::Uuid;
use crate::domain::commands::member_cmd::ReferralEvent;
use crate::domain::entities::member;
use crate::domain::messages::{MemberCreatedEvent, MemberType};
use crate::infra::repositories::member_mutation::MemberMutation;

pub struct MemberService;

impl MemberService {
    /// Create a new Member
    pub async fn create_referral(user_id: Uuid, event: MemberCreatedEvent) -> Result<ReferralEvent, Status> {
        let form_data: member::Model = member::Model {
            user_id,
            user_name: event.user_name,
            member_type: event.member_type,
            member_id: event.member_id,
            login_creds: event.login_creds,
            level: event.level,
            my_referrer_code: event.my_referrer_code,
            referee_code: event.referee_code,
            hierarchy: 0,
            active: true,
            created_at: Local::now().naive_local(),
            updated_at: Local::now().naive_local(),
            ..Default::default()
        };
        match MemberMutation::create_member(form_data).await {
            Ok(_) => Ok(ReferralEvent::Created),
            Err(_) => Err(Status::internal("Failed to create"))
        }
    }

    /// Update member profile
    pub async fn update_referral(user_id: Uuid, member_type: MemberType, level: i32) -> Result<ReferralEvent, Status> {
        // match RelationshipOrmQuery::find_by_user(user_id).await? {
        //     None => Err(Status::not_found("没有找到")),
        //     Some(model) => {
        //         let form_data = relationship::Model { sys_vip_type, available_count, ..model };
        //         let res = RelationshipOrmMutation::update_by_id(model.id, form_data).await?;
        //         Ok(ReferralEvent::Updated { model: res })
        //     }
        // }
        todo!("")
    }
}
