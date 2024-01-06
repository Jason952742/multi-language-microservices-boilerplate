use chrono::Utc;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use serde_json::Value;
use uuid::Uuid;
use crate::domain::aggregates::account_ar::{Account, AccountCommand, AccountEvent};
use crate::domain::entities::enums::{AggregateType, CurrencyType};
use crate::domain::entities::eventsource;
use crate::domain::entities::valobj::Payment;

// External services must be called during the processing of the command.
pub struct AtmError;

pub struct CheckingError;

// A very simple "happy path" set of services that always succeed.
pub struct AccountServices;

impl AccountServices {
    pub async fn create_event(account_id: &Uuid, user_id: &Uuid, txn_id: &Uuid) -> eventsource::Model {
        let account = Account::new(&account_id);
        let cmd = AccountCommand::OpenAccount { account_id: *account_id, user_id: *user_id, currency_type: CurrencyType::EUR };
        let event = account.handle(cmd).await.unwrap();
        let payload: Value = event.clone().into();
        generate_event(*account_id, Some(*txn_id), payload, event)
    }

    pub async fn deposit_event(account: &Account, payment: Payment) -> (eventsource::Model, Decimal) {
        let cmd = AccountCommand::DepositMoney { amount: payment.amount };
        let event = account.handle(cmd).await.unwrap();
        let balance = match event {
            AccountEvent::CustomerDepositedMoney { balance, .. } => balance,
            _ => dec!(0)
        };
        let payload: Value = event.clone().into();
        let es = generate_event(account.account_id, None, payload, event);
        (es, balance)
    }

    pub async fn withdraw_event(account: &Account, payment: Payment) -> (eventsource::Model, Decimal) {
        let cmd = AccountCommand::WithdrawMoney { amount: payment.amount, atm_id: payment.equipment_id };
        let event = account.handle(cmd).await.unwrap();
        let balance = match event {
            AccountEvent::CustomerWithdrewCash { balance, .. } => balance,
            _ => dec!(0)
        };
        let payload: Value = event.clone().into();
        let es = generate_event(account.account_id, None, payload, event);
        (es, balance)
    }

    pub async fn atm_withdrawal_atm_id(_atm_id: &str, _amount: Decimal) -> Result<(), AtmError> {
        Ok(())
    }

    pub async fn validate_check(_account_id: &Uuid, _check_number: &str) -> Result<(), CheckingError> {
        Ok(())
    }
}

fn generate_event(aggregate_id: Uuid, txn_id: Option<Uuid>, payload: Value, event: AccountEvent) -> eventsource::Model {
    eventsource::Model {
        id: Uuid::new_v4(),
        txn_id,
        aggregate_id,
        aggregate_type: AggregateType::Account,
        sequence: Utc::now().timestamp(),
        event_type: event.event_type(),
        event_version: event.event_version(),
        payload: payload.to_string(),
        created_at: Utc::now(),
        ..Default::default()
    }
}
