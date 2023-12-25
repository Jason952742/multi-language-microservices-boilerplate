use std::time::Duration;
use amqprs::BasicProperties;
use amqprs::callbacks::{DefaultChannelCallback, DefaultConnectionCallback};
use amqprs::channel::{BasicConsumeArguments, BasicPublishArguments, Channel, QueueBindArguments, QueueDeclareArguments};
use amqprs::connection::{Connection, OpenConnectionArguments};
use amqprs::consumer::DefaultConsumer;


#[derive(Debug)]
pub struct Rabbitmq;

impl Rabbitmq {
    pub async fn connection() -> Connection {
        // open a connection to RabbitMQ server
        let connection = Connection::open(&OpenConnectionArguments::new(
            "localhost", 5672, "rabbit", "rabbitpassword"
        )).await.unwrap();
        connection.register_callback(DefaultConnectionCallback).await.unwrap();
        connection
    }

    pub async fn channel(connection: &Connection) -> Channel {
        // open a channel on the connection
        let channel = connection.open_channel(None).await.unwrap();
        channel.register_callback(DefaultChannelCallback).await.unwrap();
        channel
    }

    pub async fn consume(channel: &Channel, queue_name: &str, consumer_tag: &str) -> String {
        let args = BasicConsumeArguments::new(&queue_name, consumer_tag);
        channel.basic_consume(DefaultConsumer::new(args.no_ack), args).await.unwrap()
    }

    pub async fn publish(channel: &Channel, exchange_name: &str, routing_key: &str, content: Vec<u8>) -> () {
        let args = BasicPublishArguments::new(exchange_name, routing_key);
        channel.basic_publish(BasicProperties::default(), content, args).await.unwrap();
    }

    pub async fn bind(channel: &Channel, queue_name: &str, routing_key: &str, exchange_name: &str) -> () {
        channel.queue_declare(QueueDeclareArguments::durable_client_named(queue_name, )).await.unwrap().unwrap();
        channel.queue_bind(QueueBindArguments::new(&queue_name, exchange_name, routing_key, )).await.unwrap();
    }

}


#[tokio::test]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    use tokio::time::sleep;
    use tracing_subscriber::{EnvFilter, fmt};
    use tracing_subscriber::layer::SubscriberExt;
    use tracing_subscriber::util::SubscriberInitExt;

    // construct a subscriber that prints formatted traces to stdout
    // global subscriber with log level according to RUST_LOG
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env())
        .try_init()
        .ok();

    // open a connection to RabbitMQ server
    let connection = Rabbitmq::connection().await;
    // open a channel on the connection
    let channel = Rabbitmq::channel(&connection).await;

    // bind the queue to exchange
    let queue_name = "hello.examples.basic";
    let routing_key = "hello.example";
    let exchange_name = "hello";

    let _ = Rabbitmq::bind(&channel, &queue_name, routing_key, exchange_name).await;

    //////////////////////////////////////////////////////////////////////////////
    // start consumer with given name
    let r = Rabbitmq::consume(&channel, queue_name, "example_basic_pub_sub").await;


    //////////////////////////////////////////////////////////////////////////////
    // publish message
    let content = String::from(
        r#"
            {
                "publisher": "example"
                "data": "Hello, Rust Rabbit!"
            }
        "#,
    ).into_bytes();

    // create arguments for basic_publish
    Rabbitmq::publish(&channel, exchange_name, routing_key, content).await;

    // keep the `channel` and `connection` object from dropping before pub/sub is done.
    // channel/connection will be closed when drop.
    sleep(Duration::from_secs(1)).await;
    // explicitly close
    channel.close().await.unwrap();
    connection.close().await.unwrap();

    Ok(())
}