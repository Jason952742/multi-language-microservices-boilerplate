use tonic::Status;
use uuid::Uuid;
use shared::GrpcStatusTool;
use crate::domain::aggregates::referral_ar::{Referral, ReferralEvent};
use crate::infra::repositories::eventsource_query::EventSourceDbQuery;

pub struct ReferralQuery;

impl ReferralQuery {
    pub async fn load(aggregate_id: Uuid) -> Result<Option<Referral>, Status> {
        let events = EventSourceDbQuery::get_eventsources(Referral::TABLE_NAME, aggregate_id)
            .await.map_err(|e| GrpcStatusTool::invalid(e.to_string().as_str()))?;
        match events.is_empty() {
            true => Ok(None),
            false => {
                let mut model = Referral::new(&aggregate_id);
                events.into_iter().for_each( |x| {
                    let json: serde_json::Value = serde_json::from_str(&*x.payload).unwrap();
                    let e = ReferralEvent::from(json);
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

    let model = ReferralQuery::load(id).await?;

    println!("{:?}", model);

    Ok(())
}