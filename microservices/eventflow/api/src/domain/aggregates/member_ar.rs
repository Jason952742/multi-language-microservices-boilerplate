use std::fmt::{Display, Formatter};
use std::ops::Add;
use chrono::{DateTime, Duration, Utc};
use serde_derive::{Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;
use crate::domain::entities::valobj::Payment;

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct Member {
    pub member_id: Uuid,
    pub user_id: Uuid,
    pub user_name: String,
    pub total_points: i32,
    pub end_date: DateTime<Utc>,
}

impl Member {
    pub const TABLE_NAME: &'static str = "member_events";

    pub fn new(id: &Uuid) -> Member {
        Self { member_id: *id, end_date: Utc::now().add(Duration::minutes(10)), ..Default::default() }
    }

    // The aggregate logic goes here. Note that this will be the _bulk_ of a CQRS system
    // so expect to use helper functions elsewhere to keep the code clean.
    pub async fn handle(&self, command: MemberCommand) -> Result<MemberEvent, MemberError> {
        match command {
            MemberCommand::RegisterMember { member_id, user_id, user_name } => {
                Ok(MemberEvent::MemberRegistered { member_id, user_id, user_name })
            }
            MemberCommand::MemberSubscribe { payments, duration } => {
                let end_date = self.end_date.add(Duration::days(duration));

                Ok(MemberEvent::MemberSubscribed { payments, duration, end_date })
            }
            MemberCommand::RewardDuration { duration } => {
                let end_date = self.end_date.add(Duration::days(duration));

                Ok(MemberEvent::DurationRewarded { duration, end_date })
            }
            MemberCommand::RewardPoints { point } => {
                let total_points = self.total_points + point;

                Ok(MemberEvent::PointsRewarded { point, total_points })
            }
        }
    }

    /// Reconstructing the domain model
    pub fn apply(&mut self, event: MemberEvent) {
        match event {
            MemberEvent::MemberRegistered { member_id, user_id, user_name } => {
                self.member_id = member_id;
                self.user_id = user_id;
                self.user_name = user_name
            }
            MemberEvent::MemberSubscribed { end_date, .. } => {
                self.end_date = end_date;
            }
            MemberEvent::DurationRewarded { end_date, .. } => {
                self.end_date = end_date;
            }
            MemberEvent::PointsRewarded { total_points, .. } => {
                self.total_points = total_points;
            }
        }
    }
}


#[derive(Debug, Serialize, Deserialize)]
pub enum MemberCommand {
    RegisterMember { member_id: Uuid, user_id: Uuid, user_name: String },
    MemberSubscribe { payments: Vec<Payment>, duration: i64 },
    RewardDuration { duration: i64 },
    RewardPoints { point: i32 },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MemberEvent {
    MemberRegistered { member_id: Uuid, user_id: Uuid, user_name: String },
    MemberSubscribed { payments: Vec<Payment>, duration: i64, end_date: DateTime<Utc> },
    DurationRewarded { duration: i64, end_date: DateTime<Utc> },
    PointsRewarded { point: i32, total_points: i32 }
}

impl MemberEvent {
    pub fn event_type(&self) -> String {
        match self {
            MemberEvent::MemberRegistered { .. } => "MemberRegistered".to_string(),
            MemberEvent::MemberSubscribed { .. } => "MemberSubscribed".to_string(),
            MemberEvent::DurationRewarded { .. } => "DurationRewarded".to_string(),
            MemberEvent::PointsRewarded { .. } => "PointsRewarded".to_string(),
        }
    }

    pub fn event_version(&self) -> String {
        "1.0".to_string()
    }
}

impl Into<serde_json::Value> for MemberEvent {
    fn into(self) -> serde_json::Value {
        serde_json::to_value(&json!(self)).expect("Error decoding payload")
    }
}

impl From<serde_json::Value> for MemberEvent {
    fn from(v: serde_json::Value) -> Self {
        serde_json::from_value::<MemberEvent>(v).unwrap()
    }
}

#[derive(Debug)]
pub struct MemberError(String);

impl From<&str> for MemberError {
    fn from(msg: &str) -> Self {
        Self(msg.to_string())
    }
}

impl Display for MemberError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for MemberError {}
