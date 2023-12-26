use shared::rabbitmq::Rabbitmq;

#[derive(Debug)]
pub struct ReferralPub;

impl ReferralPub {
    pub async fn publish() -> Result<(), Box<dyn std::error::Error>> {
        tokio::task::spawn({
            async move {
                let connection = Rabbitmq::connection().await;
                let channel = Rabbitmq::channel(&connection).await;

                for _ in 0..5 {
                    let _ = Rabbitmq::send(&channel, "mytest", "queue_test", b"Hello world!").await;
                }
                std::future::pending::<()>().await;

                Ok::<(), async_nats::Error>(())
            }
        });

        Ok(())
    }
}
