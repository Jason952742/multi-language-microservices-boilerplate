use serde::{Deserialize, Serialize};
use rust_decimal::Decimal;
use tokio::sync::oneshot;
use tonic::Status;
use uuid::Uuid;
use crate::domain::entities::valobj::{Payment, User};

pub type Response = oneshot::Sender<Result<EventflowEvent, Status>>;

#[derive(Debug)]
pub enum EventflowCommand {
    CreateUser { user_id: Uuid, user_name: String, data: String, resp: Response },
    AccountDeposit { account_id: Uuid, payment: Payment, resp: Response },
    AccountWithdraw { account_id: Uuid, payment: Payment, resp: Response },
    MemberSubscribe { member_id: Uuid, payments: Vec<Payment>, duration: i32, resp: Response },
}


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum EventflowEvent {
    Created { user: User },
    AccountDeposited { account_id: Uuid, balance: Decimal },
    AccountWithdrew { account_id: Uuid, balance: Decimal },
    MemberSubscribed { member_id: Uuid, duration: i32 },
}
