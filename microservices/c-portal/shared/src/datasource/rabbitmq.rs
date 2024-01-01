use std::env;
use lapin::{options::*, publisher_confirm::Confirmation, types::FieldTable, BasicProperties, Connection, ConnectionProperties, Channel, Queue, Consumer};
use tokio::sync::OnceCell;
use tracing::info;

#[derive(Debug)]
pub struct RabbitPool;

static CLIENT: OnceCell<Connection> = OnceCell::const_new();

impl RabbitPool {

    pub async fn connection() -> &'static Connection {
        CLIENT
            .get_or_init(|| async {
                dotenvy::dotenv().ok();
                let uri = env::var("AMQP_ADDR").expect("AMQP_ADDR must be set");
                // Use tokio executor and reactor.
                // At the moment the reactor is only available for unix.
                let options = ConnectionProperties::default()
                    .with_executor(tokio_executor_trait::Tokio::current())
                    .with_reactor(tokio_reactor_trait::Tokio);
                let connection = Connection::connect(&uri, options)
                    .await.expect("Connection failed");
                info!("RABBITMQ CONNECTED");
                connection
            })
            .await
    }

    pub async fn channel(conn: &Connection) -> Channel {
        conn.create_channel().await.expect("create channel failed")
    }

    pub async fn queue(channel: &Channel, queue_name: &str, exchange: &str, routing_key: &str) -> Queue {
        let q = channel
            .queue_declare(queue_name, QueueDeclareOptions::default(), FieldTable::default())
            .await.expect("queue_declare failed");

        channel.queue_bind(queue_name, exchange, routing_key, QueueBindOptions::default(), FieldTable::default()).await.expect("TODO: panic message");

        q
    }

    pub async fn consumer(channel: &Channel, queue: &str, consumer_tag: &str) -> Consumer {
        channel
            .basic_consume(queue, consumer_tag, BasicConsumeOptions::default(), FieldTable::default())
            .await.expect("consumer create failed")
    }

    pub async fn send(channel: &Channel, exchange: &str, routing_key: &str, payload: &[u8]) -> Confirmation {
        info!("will publish");
        channel
            .basic_publish(exchange, routing_key, BasicPublishOptions::default(), payload, BasicProperties::default())
            .await.expect("basic_publish failed")
            .await.expect("publisher-confirms failed")
    }
}


#[tokio::test]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}

#[tokio::test]
async fn tokio_test() -> Result<(), Box<dyn std::error::Error>> {
    use std::time::Duration;
    use lapin::message::DeliveryResult;

    let connection = RabbitPool::connection().await;
    let channel = RabbitPool::channel(&connection).await;
    let _queue = RabbitPool::queue(&channel, "queue_test", "wo", "mc").await;
    let consumer = RabbitPool::consumer(&channel, "queue_test", "tag_foo").await;

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
        info!(message=?delivery, "consumer received message");

        delivery
            .ack(BasicAckOptions::default())
            .await.expect("Failed to ack send_webhook_event message");
    });


    for _ in 0..10 {
        let _ = RabbitPool::send(&channel, "", "queue_test", b"Hello world!").await;
    }

    // std::future::pending::<()>().await;

    tokio::time::sleep(Duration::from_secs(5)).await;

    Ok(())
}
