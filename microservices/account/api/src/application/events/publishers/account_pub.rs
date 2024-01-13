use shared::{lapin};
use shared::datasource::rabbitmq::RabbitPool;

#[derive(Debug)]
pub struct AccountPub;

impl AccountPub {
    pub async fn _publish() -> Result<(), Box<dyn std::error::Error>> {
        tokio::task::spawn({
            async move {
                let connection = RabbitPool::connection().await;
                let channel = RabbitPool::channel(&connection).await;

                for _ in 0..5 {
                    let _ = RabbitPool::send(&channel, "mytest", "queue_test", b"Hello world!").await;
                }
                std::future::pending::<()>().await;

                Ok::<(), lapin::Error>(())
            }
        });

        Ok(())
    }
}
