use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use tokio::sync::oneshot;
use tonic::Status;
use uuid::Uuid;
use crate::domain::messages::{MemberReferralMsg};

pub type Response = oneshot::Sender<Result<MemberEvent, Status>>;

#[derive(Debug)]
pub enum MemberCommand {
    Create { user_id: Uuid, event: MemberReferralMsg, resp: Response },
    Update { user_id: Uuid, description: String, resp: Response },
    Bind { user_id: Uuid, referrer_id: Uuid, resp: Response },
}


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MemberEvent {
    Created,
    Updated,
    Bound,
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
