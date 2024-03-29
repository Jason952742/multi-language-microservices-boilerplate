use tonic::Status;
use uuid::Uuid;
use shared::utils::GrpcStatusTool;
use crate::domain::aggregates::account_ar::{Account, AccountEvent};
use crate::infra::repositories::eventsource_query::EventSourceDbQuery;

pub struct AccountQuery;

impl AccountQuery {
    pub async fn load(aggregate_id: Uuid) -> Result<Option<Account>, Status> {
        let events = EventSourceDbQuery::get_eventsources(Account::TABLE_NAME, aggregate_id).await.map_err(|e| GrpcStatusTool::invalid(e.to_string().as_str()))?;
        match events.is_empty() {
            true => Ok(None),
            false => {
                let mut model = Account::new(&aggregate_id);
                events.into_iter().for_each( |x| {
                    let json: serde_json::Value = serde_json::from_str(&*x.payload).unwrap();
                    let e = AccountEvent::from(json);
                    model.apply(e)
                });
                Ok(Some(model))
            }
        }
    }
}

#[tokio::test]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    use std::str::FromStr;

    let id = Uuid::from_str("ae053855-9321-404c-a3ce-b57e155487cf").unwrap();

    let model = AccountQuery::load(id).await?;

    println!("{:?}", model);

    Ok(())
}