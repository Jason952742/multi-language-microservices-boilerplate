use tracing::log;
use shared::{lapin};
use shared::datasource::rabbitmq::RabbitPool;
use crate::domain::messages::{MemberCreatedMsg};

#[derive(Debug)]
pub struct MemberPub;

impl MemberPub {
    pub async fn publish_member(msg: MemberCreatedMsg) -> Result<(), lapin::Error> {
        let event_name = "member_created";
        let connection = RabbitPool::connection().await;
        let channel = RabbitPool::channel(&connection).await;
        let _queue = RabbitPool::queue(&channel, &event_name, "multi_lang", "member").await;

        tokio::task::spawn({
            async move {
                let bytes: Vec<u8> = msg.into();
                let confirm = RabbitPool::send(&channel, "multi_lang", "member", bytes.as_slice()).await;

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
