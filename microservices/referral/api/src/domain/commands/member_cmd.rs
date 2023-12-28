use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use tokio::sync::oneshot;
use tonic::Status;
use uuid::Uuid;
use crate::domain::messages::{MemberCreatedEvent, MemberType};

pub type Response = oneshot::Sender<Result<ReferralEvent, Status>>;

#[derive(Debug)]
pub enum ReferralCommand {
    Create { user_id: Uuid, event: MemberCreatedEvent, resp: Response },
    Update { user_id: Uuid, member_type: MemberType, level: i32, resp: Response }
}


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ReferralEvent {
    Created,
    Updated
}


#[derive(Debug)]
pub struct ReferralError(String);

impl From<&str> for ReferralError {
    fn from(msg: &str) -> Self {
        Self(msg.to_string())
    }
}

impl Display for ReferralError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for ReferralError {}
