use serde::{Deserialize, Serialize};
use std::fmt::{Display};
use tokio::sync::oneshot;
use tonic::Status;
use uuid::Uuid;
use crate::domain::entities::valobj::{Payment, User};

pub type Response = oneshot::Sender<Result<EventflowEvent, Status>>;

#[derive(Debug)]
pub enum EventflowCommand {
    CreateUser { user_id: Uuid, user_name: String, resp: Response },
    AccountDeposit { user_id: Uuid, account_id: Uuid, payment: Payment, resp: Response },
    AccountWithdraw { user_id: Uuid, account_id: Uuid, payment: Payment, resp: Response },
    MemberSubscribe { user_id: Uuid, member_id: Uuid, payments: Vec<Payment>, duration: i32, resp: Response },
}


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum EventflowEvent {
    Created { user: User },
    AccountDeposited { user_id: Uuid, account_id: Uuid, payment: Payment },
    AccountWithdrew { user_id: Uuid, account_id: Uuid, payment: Payment },
    MemberSubscribed { user_id: Uuid, member_id: Uuid, duration: i32 },
}
