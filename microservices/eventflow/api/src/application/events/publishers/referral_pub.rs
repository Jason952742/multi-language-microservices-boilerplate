use tracing::log;
use shared::{lapin};
use shared::rabbitmq::RabbitPool;
use crate::domain::messages::MemberReferralMsg;

#[derive(Debug)]
pub struct ReferralPub;

impl ReferralPub {
    pub async fn publish_member(msg: MemberReferralMsg) -> Result<(), lapin::Error> {
        let event_name = "member_referral";
        let connection = RabbitPool::connection().await;
        let channel = RabbitPool::channel(&connection).await;
        let _queue = RabbitPool::queue(&channel, &event_name, "multi_lang", "referral").await;

        tokio::task::spawn({
            async move {
                let bytes: Vec<u8> = msg.into();
                let confirm = RabbitPool::send(&channel, "multi_lang", "referral", bytes.as_slice()).await;

                match confirm.take_message() {
                    None => println!("Received"),
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
