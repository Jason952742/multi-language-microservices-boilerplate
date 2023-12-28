use lapin::message::DeliveryResult;
use lapin::options::{BasicAckOptions};
use tokio::sync::{mpsc, Mutex, oneshot};
use tonic::Status;
use shared::rabbitmq::Rabbitmq;
use crate::domain::commands::member_cmd::{ReferralCommand, ReferralEvent};
use crate::domain::handlers::{ReferralActor, run_referral_actor};
use crate::domain::messages::MemberCreatedEvent;

#[derive(Clone)]
pub struct ReferralSub;

impl ReferralSub {
    pub async fn start_subscribe() -> Result<(), Box<dyn std::error::Error>> {
        let (tx, rx) = mpsc::channel(32);
        let actor = ReferralActor::new(rx);
        tokio::spawn(run_referral_actor(actor));

        Self::subscribe_member_created(tx.clone()).await.expect("TODO: panic message");

        Ok(())
    }

    /// Handle member created
    async fn subscribe_member_created(tx:  mpsc::Sender<ReferralCommand>) -> Result<(), Box<dyn std::error::Error>> {
        tokio::task::spawn(async move {
            let event_name = "member_created";
            let connection = Rabbitmq::connection().await;
            let channel = Rabbitmq::channel(&connection).await;
            let _queue = Rabbitmq::queue(&channel, &event_name, "member", "created").await;
            let consumer = Rabbitmq::consumer(&channel, &event_name, "referral-member").await;

            consumer.set_delegate(move |delivery: DeliveryResult| async move {
                let delivery = match delivery {
                    // Carries the delivery alongside its channel
                    Ok(Some(delivery)) => delivery,
                    // The consumer got canceled
                    Ok(None) => return,
                    // Carries the error and is always followed by Ok(None)
                    Err(error) => {
                        dbg!("Failed to consume queue message {}", error);
                        return;
                    }
                };
                // Do something with the delivery data (The message payload)
                let payload = MemberCreatedEvent::from(delivery.data.as_ref());
                tracing::info!("Receive {:?} Event: {:?}", &event_name, &payload);
                let user_id = payload.clone().user_id.to_string();

                let (resp_tx, resp_rx) = oneshot::channel();
                let command = ReferralCommand::Create { user_id: payload.user_id, event: payload, resp: resp_tx };

                if tx.send(command).await.is_err() {
                    tracing::info!("{:?} - {:?} failed", &event_name, &user_id);
                    eprintln!("connection task shutdown");
                }
                // handler command
                match resp_rx.await.unwrap() {
                    Ok(event) => tracing::info!("Successful handling {:?} - {:?}", event, user_id),
                    Err(e) => eprintln!("handler failed: {:?}", e)
                }
                // Ack
                delivery.ack(BasicAckOptions::default()).await.expect("Failed to ack referral_event message");
            });

            std::future::pending::<()>().await;
            Ok::<(), Box<dyn std::error::Error + Sync + Send>>(())
        });
        Ok(())
    }
}
