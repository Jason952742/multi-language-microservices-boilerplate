use lapin::message::DeliveryResult;
use lapin::options::BasicAckOptions;
use tokio::sync::{mpsc, oneshot};
use shared::rabbitmq::Rabbitmq;
use tracing::info;

#[derive(Clone)]
pub struct ReferralSub;

impl ReferralSub {
    pub async fn start_subscribe() -> Result<(), Box<dyn std::error::Error>> {
        // let (tx, rx) = mpsc::channel(32);
        // let actor = ReferralActor::new(rx);
        // tokio::spawn(run_referral_actor(actor));

        Self::subscribe_member_created().await.expect("TODO: panic message");

        Ok(())
    }

    /// Handle member created
    async fn subscribe_member_created() -> Result<(), Box<dyn std::error::Error>> {
        tokio::task::spawn({
            async move {
                let connection = Rabbitmq::connection().await;
                let channel = Rabbitmq::channel(&connection).await;
                let _queue = Rabbitmq::queue(&channel, "queue_test").await;
                let consumer = Rabbitmq::consumer(&channel, "queue_test", "tag_foo").await;

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
                    info!(message=?delivery.data, "consumer received message");

                    delivery
                        .ack(BasicAckOptions::default())
                        .await.expect("Failed to ack send_webhook_event message");
                });

                std::future::pending::<()>().await;

                Ok::<(), async_nats::Error>(())
            }
        });

        Ok(())
    }

}
