use std::fmt::{Display, Formatter};
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use serde_derive::{Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;
use crate::domain::entities::enums::CurrencyType;
use crate::domain::services::AccountServices;

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct Account {
    pub account_id: Uuid,
    pub user_id: Uuid,
    pub currency_type: CurrencyType,
    pub balance: Decimal,
}

impl Account {
    pub const TABLE_NAME: &'static str = "account_event";

    pub fn new(id: &Uuid) -> Account {
        Self { account_id: *id, ..Default::default() }
    }

    // The aggregate logic goes here. Note that this will be the _bulk_ of a CQRS system
    // so expect to use helper functions elsewhere to keep the code clean.
    pub async fn handle(&self, command: AccountCommand) -> Result<AccountEvent, AccountError> {
        match command {
            AccountCommand::OpenAccount { account_id, user_id, currency_type, } => {
                Ok(AccountEvent::AccountOpened { account_id, user_id, currency_type })
            }
            AccountCommand::DepositMoney { amount } => {
                let balance = self.balance + amount;

                Ok(AccountEvent::CustomerDepositedMoney { amount, balance })
            }
            AccountCommand::WithdrawMoney { amount, atm_id } => {
                let balance = self.balance - amount;
                if balance < dec!(0) { return Err("funds not available".into()); }
                if AccountServices::atm_withdrawal_atm_id(&atm_id, amount).await.is_err() { return Err("atm rule violation".into()); };

                Ok(AccountEvent::CustomerWithdrewCash { amount, balance })
            }
            AccountCommand::WriteCheck { check_number, amount, } => {
                let balance = self.balance - amount;
                if balance < dec!(0) { return Err("funds not available".into()); }
                if AccountServices::validate_check(&self.account_id, &check_number).await.is_err() { return Err("check invalid".into()); };

                Ok(AccountEvent::CustomerWroteCheck { check_number, amount, balance })
            }
        }
    }

    /// Reconstructing the domain model
    pub fn apply(&mut self, event: AccountEvent) {
        match event {
            AccountEvent::AccountOpened { account_id, user_id, currency_type } => {
                self.account_id = account_id;
                self.user_id = user_id;
                self.currency_type = currency_type
            }
            AccountEvent::CustomerDepositedMoney { amount: _, balance } => {
                self.balance = balance;
            }
            AccountEvent::CustomerWithdrewCash { amount: _, balance } => {
                self.balance = balance;
            }
            AccountEvent::CustomerWroteCheck { check_number: _, amount: _, balance } => {
                self.balance = balance;
            }
        }
    }
}


#[derive(Debug, Serialize, Deserialize)]
pub enum AccountCommand {
    OpenAccount { account_id: Uuid, user_id: Uuid, currency_type: CurrencyType },
    DepositMoney { amount: Decimal },
    WithdrawMoney { amount: Decimal, atm_id: String },
    WriteCheck { check_number: String, amount: Decimal },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AccountEvent {
    AccountOpened { account_id: Uuid, user_id: Uuid, currency_type: CurrencyType },
    CustomerDepositedMoney { amount: Decimal, balance: Decimal },
    CustomerWithdrewCash { amount: Decimal, balance: Decimal },
    CustomerWroteCheck { check_number: String, amount: Decimal, balance: Decimal },
}

impl AccountEvent {
    pub fn event_type(&self) -> String {
        match self {
            AccountEvent::AccountOpened { .. } => "AccountOpened".to_string(),
            AccountEvent::CustomerDepositedMoney { .. } => "CustomerDepositedMoney".to_string(),
            AccountEvent::CustomerWithdrewCash { .. } => "CustomerWithdrewCash".to_string(),
            AccountEvent::CustomerWroteCheck { .. } => "CustomerWroteCheck".to_string(),
        }
    }

    pub fn event_version(&self) -> String {
        "1.0".to_string()
    }
}

impl Into<serde_json::Value> for AccountEvent {
    fn into(self) -> serde_json::Value {
        serde_json::to_value(&json!(self)).expect("Error decoding payload")
    }
}

impl From<serde_json::Value> for AccountEvent {
    fn from(v: serde_json::Value) -> Self {
        serde_json::from_value::<AccountEvent>(v).unwrap()
    }
}

#[derive(Debug)]
pub struct AccountError(String);

impl From<&str> for AccountError {
    fn from(msg: &str) -> Self {
        Self(msg.to_string())
    }
}

impl Display for AccountError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for AccountError {}
