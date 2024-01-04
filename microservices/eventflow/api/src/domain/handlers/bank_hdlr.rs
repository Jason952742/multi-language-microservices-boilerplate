use std::collections::HashMap;
use std::sync::Arc;
use cqrs_es::AggregateError;
use cqrs_es::persist::ViewRepository;
use postgres_es::{default_postgress_pool, PostgresCqrs, PostgresViewRepository};
use crate::infra::framework::bank_fw::cqrs_framework;
use crate::domain::aggregates::bank_ar::BankAccount;
use crate::domain::commands::bank_cmd::BankAccountCommand;
use crate::domain::events::bank_evt::BankAccountError;
use crate::domain::queries::bank_qry::BankAccountView;

// Serves as our query endpoint to respond with the materialized `BankAccountView`
// for the requested account.
pub async fn query_handler(
    account_id: String,
    account_query: Arc<PostgresViewRepository<BankAccountView, BankAccount>>,
) -> Option<BankAccountView> {
    let view = account_query.load(&account_id).await.unwrap_or_else(|err| {
        println!("Error: {:#?}\n", err);
        None
    });
    view
}

// Serves as our command endpoint to make changes in a `BankAccount` aggregate.
pub async fn command_handler(
    account_id: String,
    cqrs: Arc<PostgresCqrs<BankAccount>>,
    command: BankAccountCommand,
    metadata: HashMap<String, String>
) -> Result<(), AggregateError<BankAccountError>> {

    println!("Command: {:?}", command);
    println!("Metadata: {:?}", metadata);


    let result =  cqrs.execute_with_metadata(&account_id, command, metadata).await;
    result
}

#[tokio::test]
async fn query_account() -> Result<(), Box<dyn std::error::Error>> {
    let pool = default_postgress_pool("postgresql://postgres:postgres@localhost:5432/eventflow").await;
    let (cqrs, account_query) = cqrs_framework(pool);

    let account_id = "BKKM-2a67f862".to_string();

    let res = query_handler(account_id, account_query).await;

    println!("{:?}", res);

    Ok(())
}

#[tokio::test]
async fn create_account() -> Result<(), Box<dyn std::error::Error>> {
    let pool = default_postgress_pool("postgresql://postgres:postgres@localhost:5432/eventflow").await;
    let (cqrs, account_query) = cqrs_framework(pool);

    let account_id = "BKKM-2a67f862".to_string();
    let command: BankAccountCommand = BankAccountCommand::OpenAccount { account_id: account_id.clone() };
    let mut metadata: HashMap<String, String> = HashMap::new();
    metadata.insert(String::from("key1"), String::from("value1"));
    metadata.insert(String::from("key2"), String::from("value2"));
    metadata.insert(String::from("key3"), String::from("value3"));

    let r = command_handler(account_id, cqrs, command, metadata).await;

    Ok(())
}

#[tokio::test]
async fn despoit() -> Result<(), Box<dyn std::error::Error>> {
    let pool = default_postgress_pool("postgresql://postgres:postgres@localhost:5432/eventflow").await;
    let (cqrs, account_query) = cqrs_framework(pool);

    let account_id = "BKKM-2a67f862".to_string();
    let command: BankAccountCommand = BankAccountCommand::DepositMoney { amount: 1000.0 };
    let mut metadata: HashMap<String, String> = HashMap::new();
    metadata.insert(String::from("key1"), String::from("value1"));
    metadata.insert(String::from("key2"), String::from("value2"));
    metadata.insert(String::from("key3"), String::from("value3"));

    let r = command_handler(account_id, cqrs, command, metadata).await;

    Ok(())
}

#[tokio::test]
async fn withdraw() -> Result<(), Box<dyn std::error::Error>> {
    let pool = default_postgress_pool("postgresql://postgres:postgres@localhost:5432/eventflow").await;
    let (cqrs, account_query) = cqrs_framework(pool);

    let account_id = "BKKM-2a67f862".to_string();
    let command: BankAccountCommand = BankAccountCommand::WithdrawMoney { amount: 400.0, atm_id: "eeefdsfasf".to_string() };
    let mut metadata: HashMap<String, String> = HashMap::new();
    metadata.insert(String::from("key1"), String::from("value1"));
    metadata.insert(String::from("key2"), String::from("value2"));
    metadata.insert(String::from("key3"), String::from("value3"));

    let r = command_handler(account_id, cqrs, command, metadata).await;

    Ok(())
}

#[tokio::test]
async fn write_check() -> Result<(), Box<dyn std::error::Error>> {
    let pool = default_postgress_pool("postgresql://postgres:postgres@localhost:5432/eventflow").await;
    let (cqrs, account_query) = cqrs_framework(pool);

    let account_id = "BKKM-2a67f862".to_string();
    let command: BankAccountCommand = BankAccountCommand::WriteCheck { check_number: "65421".to_string(), amount: 256.28 };
    let mut metadata: HashMap<String, String> = HashMap::new();
    metadata.insert(String::from("key1"), String::from("value1"));
    metadata.insert(String::from("key2"), String::from("value2"));
    metadata.insert(String::from("key3"), String::from("value3"));

    command_handler(account_id, cqrs, command, metadata).await;

    Ok(())
}
