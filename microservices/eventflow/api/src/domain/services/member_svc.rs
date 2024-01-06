use chrono::{DateTime, Utc};
use serde_json::Value;
use uuid::Uuid;
use crate::domain::aggregates::member_ar::{Member, MemberCommand, MemberEvent};
use crate::domain::entities::enums::{AggregateType};
use crate::domain::entities::eventsource;
use crate::domain::entities::valobj::Payment;


pub struct MemberServices;

impl MemberServices {

    pub async fn register_event(member_id: &Uuid, user_id: &Uuid, user_name: String, txn_id: &Uuid) -> eventsource::Model {
        let member = Member::new(&member_id);
        let cmd = MemberCommand::RegisterMember { member_id: *member_id, user_id: *user_id, user_name };
        let event = member.handle(cmd).await.unwrap();
        let payload: Value = event.clone().into();
        generate_event(*member_id, Some(*txn_id), payload, event)
    }

    pub async fn subscribe_event(member: &Member, payments: Vec<Payment>, duration: i64)-> (eventsource::Model, DateTime<Utc>) {
        let cmd = MemberCommand::MemberSubscribe { payments, duration };
        let event = member.handle(cmd).await.unwrap();
        let end_date = match event {
            MemberEvent::MemberSubscribed { end_date, .. } => end_date,
            _ => panic!("event error")
        };
        let payload: Value = event.clone().into();
        let es = generate_event(member.member_id, None, payload, event);
        (es, end_date)
    }

    pub async fn _reward_duration_event(member: &Member, duration: i64, txn_id: &Uuid)-> (eventsource::Model, DateTime<Utc>) {
        let cmd = MemberCommand::RewardDuration { duration };
        let event = member.handle(cmd).await.unwrap();
        let end_date = match event {
            MemberEvent::DurationRewarded { end_date, .. } => end_date,
            _ => panic!("event error")
        };
        let payload: Value = event.clone().into();
        let es = generate_event(member.member_id, Some(*txn_id), payload, event);
        (es, end_date)
    }

    pub async fn _reward_point_event(member: &Member, point: i32, txn_id: &Uuid)-> (eventsource::Model, i32) {
        let cmd = MemberCommand::RewardPoints { point };
        let event = member.handle(cmd).await.unwrap();
        let total_points = match event {
            MemberEvent::PointsRewarded { total_points, .. } => total_points,
            _ => panic!("event error")
        };
        let payload: Value = event.clone().into();
        let es = generate_event(member.member_id, Some(*txn_id), payload, event);
        (es, total_points)
    }

}

fn generate_event(aggregate_id: Uuid, txn_id: Option<Uuid>, payload: Value, event: MemberEvent) -> eventsource::Model {
    eventsource::Model {
        id: Uuid::new_v4(),
        txn_id,
        aggregate_id,
        aggregate_type: AggregateType::Member,
        sequence: Utc::now().timestamp(),
        event_type: event.event_type(),
        event_version: event.event_version(),
        payload: payload.to_string(),
        created_at: Utc::now(),
        ..Default::default()
    }
}