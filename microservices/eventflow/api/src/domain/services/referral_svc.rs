use chrono::{Utc};
use serde_json::Value;
use uuid::Uuid;

use crate::domain::aggregates::referral_ar::{Referral, ReferralCommand, ReferralEvent};
use crate::domain::entities::enums::{AggregateType};
use crate::domain::entities::eventsource;


pub struct ReferralServices;

impl ReferralServices {

    pub async fn create_referral_event(user_id: &Uuid, referral_code: &str, referrer_id: &Option<Uuid>, referrer_code: &Option<String>, txn_id: &Uuid) -> eventsource::Model {
        let user = Referral::new(&user_id);
        let cmd = ReferralCommand::CreateReferral {
            user_id: *user_id,
            referral_code: referral_code.to_string(),
            referrer_id: *referrer_id,
            referrer_code: referrer_code.to_owned(),
        };
        let event = user.handle(cmd).await.unwrap();
        let payload: Value = event.clone().into();
        generate_event(*user_id, Some(*txn_id), payload, event)
    }

    pub async fn user_registered_event(user: &Referral, referred_id: Uuid) -> eventsource::Model {
        let cmd = ReferralCommand::UserRegistered { referred_id };
        let event = user.handle(cmd).await.unwrap();
        let payload: Value = event.clone().into();
        generate_event(user.user_id, None, payload, event)
    }
}

fn generate_event(aggregate_id: Uuid, txn_id: Option<Uuid>, payload: Value, event: ReferralEvent) -> eventsource::Model {
    eventsource::Model {
        id: Uuid::new_v4(),
        txn_id,
        aggregate_id,
        aggregate_type: AggregateType::Referral,
        sequence: Utc::now().timestamp(),
        event_type: event.event_type(),
        event_version: event.event_version(),
        payload: payload.to_string(),
        created_at: Utc::now(),
        ..Default::default()
    }
}