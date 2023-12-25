use std::sync::Arc;
use lapin::{options::*, publisher_confirm::Confirmation, types::FieldTable, BasicProperties, Connection, ConnectionProperties, Channel, Queue};
use lapin::message::DeliveryResult;
use tracing::info;

#[derive(Debug)]
pub struct Rabbitmq;

impl Rabbitmq {
    pub async fn connection() -> Connection {
        let addr = std::env::var("AMQP_ADDR").unwrap_or_else(|_| "amqp://rabbit:rabbitpassword@127.0.0.1:5672/%2f".into());
        let conn = Connection::connect(&addr, ConnectionProperties::default()).await.expect("connection error");
        info!("CONNECTED");
        conn
    }

    pub async fn channel(conn: &Connection) -> Channel {
        conn.create_channel().await.expect("create_channel")
    }

    pub async fn queue(channel: &Channel, queue_name: &str) -> Queue {
        let queue = channel
            .queue_declare(queue_name, QueueDeclareOptions::default(), FieldTable::default())
            .await.expect("queue_declare");
        info!(?queue, "Declared queue");
        queue
    }

    pub async fn longlive_consume() {}

    pub async fn consume(conn: &Connection, queue: &str, consumer_tag: &str) -> () {
        // receive channel
        let channel_receive = Rabbitmq::channel(&conn).await;
        let consumer_tag = Arc::new(consumer_tag.to_string());

        let _ = &channel_receive
            .basic_consume(queue, &consumer_tag, BasicConsumeOptions::default(), FieldTable::default())
            .await.expect("basic_consume")
            .set_delegate(move |delivery: DeliveryResult| {
                let channel = channel_receive.clone();
                let consumer_tag = Arc::clone(&consumer_tag);

                async move {
                    info!(message=?delivery, "consume received message: ");
                    if let Ok(Some(delivery)) = delivery {

                        delivery
                            .ack(BasicAckOptions::default())
                            .await.expect("basic_ack");

                        channel
                            .basic_cancel(&consumer_tag, BasicCancelOptions::default())
                            .await.expect("basic_cancel");
                    }
                }
            });
    }

    pub async fn publish(channel: &Channel, exchange: &str, routing_key: &str, payload: &[u8]) -> Confirmation {
        channel
            .basic_publish(exchange, routing_key, BasicPublishOptions::default(), payload, BasicProperties::default())
            .await.expect("basic_publish")
            .await.expect("publisher-confirms")
    }
}


#[tokio::test]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info");
    }

    tracing_subscriber::fmt::init();

    async_global_executor::block_on(async {
        let conn = Rabbitmq::connection().await;

        {
            // send channel
            let channel_send = Rabbitmq::channel(&conn).await;
            // info!(state=?conn.status().state());

            // create the hello queue
            let _ = Rabbitmq::queue(&channel_send, "hello").await;

            // info!(state=?conn.status().state());

            info!("will consume");
            Rabbitmq::consume(&conn, "hello", "my_consumer").await;

            // info!(state=?conn.status().state());

            info!("will publish");
            let payload = b"Hello world!";
            let confirm = Rabbitmq::publish(&channel_send, "", "hello", payload).await;

            assert_eq!(confirm, Confirmation::NotRequested);

            // info!(state=?conn.status().state());
        }

        conn.run().expect("conn.run");
    });


    Ok(())
}