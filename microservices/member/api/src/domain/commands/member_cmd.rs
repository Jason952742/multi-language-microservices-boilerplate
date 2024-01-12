use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tokio::sync::oneshot;
use tonic::Status;
use uuid::Uuid;
use crate::domain::entities::enums::MemberType;

pub type Response = oneshot::Sender<Result<MemberEvent, Status>>;

#[derive(Debug)]
pub enum MemberCommand {
    Create { id: Uuid, user_id: Uuid, user_name: String, sub_end_date: DateTime<Utc>, resp: Response },
    Subscribe { id: Uuid, user_id: Uuid, sub_end_date: DateTime<Utc>, resp: Response },
    Disable { id: Uuid, resp: Response },
    Update { id: Uuid, member_type: MemberType, level: i32, active: bool, description: String, resp: Response },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MemberEvent {
    Created { id: Uuid },
    Subscribed,
    Updated,
    Disabled,
}
