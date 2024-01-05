use std::fmt::{Display, Formatter};
use serde_derive::{Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;
use crate::domain::entities::enums::CurrencyType;
use crate::domain::services::HappyPathAccountServices;

#[derive(Default, Serialize, Deserialize)]
pub struct Account {
    account_id: Uuid,
    user_id: Uuid,
    currency_type: CurrencyType,
    balance: f64,
}

impl Account {
    pub const TABLE_NAME: &'static str = "account_event";

    pub async fn new(account_id: Uuid, user_id: Uuid, currency_type: CurrencyType) -> Account {
        Self { account_id, user_id, currency_type, balance: 0_f64 }
    }

    // The aggregate logic goes here. Note that this will be the _bulk_ of a CQRS system
    // so expect to use helper functions elsewhere to keep the code clean.
    pub async fn handle(&self, command: AccountCommand) -> Result<Vec<AccountEvent>, AccountError> {
        match command {
            AccountCommand::OpenAccount { account_id, user_id, currency_type, } => {
                Ok(vec![
                    AccountEvent::AccountOpened { account_id, user_id, currency_type }
                ])
            }
            AccountCommand::DepositMoney { amount } => {
                let balance = self.balance + amount;

                Ok(vec![AccountEvent::CustomerDepositedMoney { amount, balance }])
            }
            AccountCommand::WithdrawMoney { amount, atm_id } => {
                let balance = self.balance - amount;
                if balance < 0_f64 { return Err("funds not available".into()); }
                if HappyPathAccountServices::atm_withdrawal_atm_id(&atm_id, amount).await.is_err() { return Err("atm rule violation".into()); };

                Ok(vec![AccountEvent::CustomerWithdrewCash { amount, balance }])
            }
            AccountCommand::WriteCheck { check_number, amount, } => {
                let balance = self.balance - amount;
                if balance < 0_f64 { return Err("funds not available".into()); }
                if HappyPathAccountServices::validate_check(&self.account_id, &check_number).await.is_err() { return Err("check invalid".into()); };

                Ok(vec![AccountEvent::CustomerWroteCheck { check_number, amount, balance }])
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
    DepositMoney { amount: f64 },
    WithdrawMoney { amount: f64, atm_id: String },
    WriteCheck { check_number: String, amount: f64 },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AccountEvent {
    AccountOpened {
        account_id: Uuid,
        user_id: Uuid,
        currency_type: CurrencyType,
    },
    CustomerDepositedMoney {
        amount: f64,
        balance: f64,
    },
    CustomerWithdrewCash {
        amount: f64,
        balance: f64,
    },
    CustomerWroteCheck {
        check_number: String,
        amount: f64,
        balance: f64,
    },
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

#[tokio::test]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    let e = AccountEvent::AccountOpened { user_id: Uuid::new_v4(), account_id: Uuid::new_v4(), currency_type: Default::default() };

    let s: serde_json::Value = e.into();

    println!("{}", s);
    
    Ok(())
}