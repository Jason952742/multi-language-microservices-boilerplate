use serde::{Deserialize, Serialize};
use tokio::sync::oneshot;
use tonic::Status;
use uuid::Uuid;
use crate::domain::entities::enums::{AccountType, CurrencyType};

pub type Response = oneshot::Sender<Result<AccountEvent, Status>>;

#[derive(Debug)]
pub enum AccountCommand {
    Create { id: Uuid, user_id: Uuid, ccy_type: CurrencyType, resp: Response },
    Disable { id: Uuid, resp: Response },
    Update { id: Uuid, account_type: AccountType, account_name: String, description: String, resp: Response },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AccountEvent {
    Created { id: Uuid },
    Updated,
    Disabled,
}
