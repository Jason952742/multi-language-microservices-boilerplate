use std::time::Duration;
use lapin::{options::*, publisher_confirm::Confirmation, types::FieldTable, BasicProperties, Connection, ConnectionProperties, Channel, Queue, Consumer};
use lapin::message::DeliveryResult;
use tracing::info;

#[derive(Debug)]
pub struct Rabbitmq;

impl Rabbitmq {
    pub async fn connection() -> Connection {
        let uri = "amqp://rabbit:rabbitpassword@127.0.0.1:5672/%2f";
        // Use tokio executor and reactor.
        // At the moment the reactor is only available for unix.
        let options = ConnectionProperties::default()
            .with_executor(tokio_executor_trait::Tokio::current())
            .with_reactor(tokio_reactor_trait::Tokio);
        let connection = Connection::connect(uri.into(), options)
            .await.expect("Connection failed");
        info!("RABBITMQ CONNECTED");
        connection
    }

    pub async fn channel(conn: &Connection) -> Channel {
        conn.create_channel().await.expect("create channel failed")
    }

    pub async fn queue(channel: &Channel, queue_name: &str) -> Queue {
        channel
            .queue_declare(queue_name, QueueDeclareOptions::default(), FieldTable::default())
            .await.expect("queue_declare failed")
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
    let connection = Rabbitmq::connection().await;
    let channel = Rabbitmq::channel(&connection).await;
    let _queue = Rabbitmq::queue(&channel, "queue_test").await;
    let consumer = Rabbitmq::consumer(&channel, "queue_test", "tag_foo").await;

    // consumer.set_delegate(move |delivery: DeliveryResult| async move {
    //     let delivery = match delivery {
    //         // Carries the delivery alongside its channel
    //         Ok(Some(delivery)) => delivery,
    //         // The consumer got canceled
    //         Ok(None) => return,
    //         // Carries the error and is always followed by Ok(None)
    //         Err(error) => {
    //             dbg!("Failed to consume queue message {}", error);
    //             return;
    //         }
    //     };
    //
    //     // Do something with the delivery data (The message payload)
    //     info!(message=?delivery, "consumer received message");
    //
    //     delivery
    //         .ack(BasicAckOptions::default())
    //         .await.expect("Failed to ack send_webhook_event message");
    // });


    for _ in 0..10 {
        let _ = Rabbitmq::send(&channel, "", "queue_test", b"Hello world!").await;
    }

    // std::future::pending::<()>().await;

    tokio::time::sleep(Duration::from_secs(5)).await;

    Ok(())
}
