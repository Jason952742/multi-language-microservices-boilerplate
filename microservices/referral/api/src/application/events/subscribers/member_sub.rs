use futures::TryStreamExt;
use futures_lite::StreamExt;
use lapin::options::{BasicAckOptions};
use tokio::sync::{mpsc, oneshot};
use shared::lapin;
use shared::datasource::rabbitmq::RabbitPool;
use crate::domain::commands::member_cmd::{MemberCommand};
use crate::domain::handlers::{MemberActor, run_member_actor};
use crate::domain::messages::MemberReferralMsg;

#[derive(Clone)]
pub struct MemberSub;

impl MemberSub {
    pub async fn start_subscribe() -> anyhow::Result<()> {
        let (tx, rx) = mpsc::channel(32);
        let actor = MemberActor::new(rx);
        tokio::spawn(run_member_actor(actor));

        Self::subscribe_member_created(tx.clone()).await.expect("TODO: panic message");

        Ok(())
    }

    /// Handle member created
    pub async fn subscribe_member_created(tx: mpsc::Sender<MemberCommand>) -> Result<(), lapin::Error> {
        let event_name = "member_referral";
        let connection = RabbitPool::connection().await;
        let channel = RabbitPool::channel(&connection).await;
        let _queue = RabbitPool::queue(&channel, &event_name, "multi_lang", "referral").await;
        let consumer = RabbitPool::consumer(&channel, &event_name, "referral-consumer").await;
        let mut consumer_stream = consumer.into_stream();

        tokio::task::spawn(async move {
            while let Some(delivery) = consumer_stream.next().await {
                if let Ok(delivery) = delivery {
                    // Do something with the delivery data (The message payload)
                    let payload = MemberReferralMsg::from(delivery.data.as_ref());
                    tracing::info!("Receive {:?} Event: {:?}", &event_name, &payload);
                    let user_id = payload.clone().user_id.to_string();

                    let (resp_tx, resp_rx) = oneshot::channel();
                    let command = MemberCommand::Create { user_id: payload.user_id, event: payload, resp: resp_tx };

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
                    channel.basic_ack(delivery.delivery_tag, BasicAckOptions::default()).await?;
                }
            }

            Ok::<(), lapin::Error>(())
        });

        // tokio::signal::ctrl_c().await.expect("Failed to");
        Ok(())
    }
}
