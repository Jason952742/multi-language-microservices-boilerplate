use crate::domain::commands::account_cmd::AccountCommand;
use crate::domain::handlers::{run_account_actor, AccountActor};
use crate::domain::messages::AccountCreatedMsg;
use futures::TryStreamExt;
use futures_lite::StreamExt;
use lapin::options::BasicAckOptions;
use shared::datasource::rabbitmq::RabbitPool;
use shared::lapin;
use tokio::sync::{mpsc, oneshot};

#[derive(Clone)]
pub struct AccountSub;

impl AccountSub {
  pub async fn start_subscribe() -> anyhow::Result<()> {
    let (tx, rx) = mpsc::channel(32);
    let actor = AccountActor::new(rx);
    tokio::spawn(run_account_actor(actor));

    Self::subscribe_account_created(tx.clone()).await.expect("TODO: panic message");

    Ok(())
  }

  /// Handle account created
  pub async fn subscribe_account_created(tx: mpsc::Sender<AccountCommand>) -> Result<(), lapin::Error> {
    let event_name = "account_created";
    let connection = RabbitPool::connection().await;
    let channel = RabbitPool::channel(&connection).await;
    let _queue = RabbitPool::queue(&channel, &event_name, "multi_lang", "account").await;
    let consumer = RabbitPool::consumer(&channel, &event_name, "account-consumer").await;
    let mut consumer_stream = consumer.into_stream();

    tokio::task::spawn(async move {
      while let Some(delivery) = consumer_stream.next().await {
        if let Ok(delivery) = delivery {
          // Do something with the delivery data (The message payload)
          let payload = AccountCreatedMsg::from(delivery.data.as_ref());
          tracing::info!("Receive {:?} Event: {:?}", &event_name, &payload);
          let user_id = payload.clone().user_id.to_string();

          let (resp_tx, resp_rx) = oneshot::channel();
          let command = AccountCommand::Create { id: payload.account_id, user_id: payload.user_id, ccy_type: payload.ccy_type, resp: resp_tx };

          if tx.send(command).await.is_err() {
            tracing::info!("{:?} - {:?} failed", &event_name, &user_id);
            eprintln!("connection task shutdown");
          }
          // handler command
          match resp_rx.await.unwrap() {
            Ok(event) => tracing::info!("Successful handling {:?} - {:?}", event, user_id),
            Err(e) => eprintln!("handler failed: {:?}", e),
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
