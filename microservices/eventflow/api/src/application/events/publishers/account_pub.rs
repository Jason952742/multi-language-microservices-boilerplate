use tracing::log;
use shared::{lapin};
use shared::datasource::rabbitmq::RabbitPool;
use crate::domain::messages::{AccountCreatedMsg};

#[derive(Debug)]
pub struct AccountPub;

impl AccountPub {
    pub async fn publish_account(msg: AccountCreatedMsg) -> Result<(), lapin::Error> {
        let event_name = "account_created";
        let connection = RabbitPool::connection().await;
        let channel = RabbitPool::channel(&connection).await;
        let _queue = RabbitPool::queue(&channel, &event_name, "multi_lang", "account").await;

        tokio::task::spawn({
            async move {
                let bytes: Vec<u8> = msg.into();
                let confirm = RabbitPool::send(&channel, "multi_lang", "account", bytes.as_slice()).await;

                match confirm.take_message() {
                    None => println!("Member Created Message Received!"),
                    Some(message) => {
                        let error = message.error();

                        log::info!("send message {:?} , error: {:?}", message.data, error);
                    }
                }
                Ok::<(), lapin::Error>(())
            }
        });

        Ok(())
    }
}
