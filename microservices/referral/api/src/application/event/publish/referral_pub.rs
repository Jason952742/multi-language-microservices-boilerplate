use shared::rabbitmq::Rabbitmq;

#[derive(Debug)]
pub struct ReferralPub;

impl ReferralPub {
    pub async fn publish() -> Result<(), Box<dyn std::error::Error>> {
        tokio::task::spawn({
            async move {
                let connection = Rabbitmq::connection().await;
                let channel = Rabbitmq::channel(&connection).await;
                let _queue = Rabbitmq::queue(&channel, "queue_test").await;

                for _ in 0..3 {
                    let _ = Rabbitmq::send(&channel, "", "queue_test", b"Hello world!").await;
                }
                std::future::pending::<()>().await;

                Ok::<(), async_nats::Error>(())
            }
        });

        Ok(())
    }
}
