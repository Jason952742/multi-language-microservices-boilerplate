use std::time::Duration;
use amqprs::{
    callbacks::{ChannelCallback, ConnectionCallback},
    channel::Channel, connection::{Connection, OpenConnectionArguments},
    Ack, BasicProperties, Cancel, Close, CloseChannel, Nack, Return,
};
use amqprs::channel::{BasicConsumeArguments, BasicPublishArguments, QueueBindArguments, QueueDeclareArguments};
use amqprs::consumer::DefaultConsumer;
use async_trait::async_trait;

////////////////////////////////////////////////////////////////////////////////
type Result<T> = std::result::Result<T, amqprs::error::Error>;

////////////////////////////////////////////////////////////////////////////////
struct ExampleConnectionCallback;

#[allow(unused_variables, /* template */)]
#[async_trait]
impl ConnectionCallback for ExampleConnectionCallback {
    async fn close(&mut self, connection: &Connection, close: Close) -> Result<()> {
        Ok(())
    }

    async fn blocked(&mut self, connection: &Connection, reason: String) {}
    async fn unblocked(&mut self, connection: &Connection) {}
}

////////////////////////////////////////////////////////////////////////////////
struct ExampleChannelCallback;

#[allow(unused_variables, /* template */)]
#[async_trait]
impl ChannelCallback for ExampleChannelCallback {
    async fn close(&mut self, channel: &Channel, close: CloseChannel) -> Result<()> {
        Ok(())
    }
    async fn cancel(&mut self, channel: &Channel, cancel: Cancel) -> Result<()> {
        Ok(())
    }
    async fn flow(&mut self, channel: &Channel, active: bool) -> Result<bool> {
        Ok(true)
    }
    async fn publish_ack(&mut self, channel: &Channel, ack: Ack) {}
    async fn publish_nack(&mut self, channel: &Channel, nack: Nack) {}
    async fn publish_return(
        &mut self,
        channel: &Channel,
        ret: Return,
        basic_properties: BasicProperties,
        content: Vec<u8>,
    ) {}
}

#[derive(Debug)]
pub struct Rabbitmq;

impl Rabbitmq {
    pub async fn connection() -> Connection {
        // open a connection to RabbitMQ server
        let args = OpenConnectionArguments::new("localhost", 5672, "rabbit", "rabbitpassword");
        let connection = Connection::open(&args).await.unwrap();
        connection.register_callback(ExampleConnectionCallback).await.unwrap();
        connection
    }

    pub async fn channel(connection: &Connection) -> Channel {
        // open a channel on the connection
        let channel = connection.open_channel(None).await.unwrap();
        channel.register_callback(ExampleChannelCallback).await.unwrap();
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
        channel.queue_declare(QueueDeclareArguments::durable_client_named(queue_name)).await.unwrap().unwrap();
        channel.queue_bind(QueueBindArguments::new(&queue_name, exchange_name, routing_key)).await.unwrap();
    }
}


#[tokio::test]
async fn main() -> Result<()> {
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
    let _ = Rabbitmq::consume(&channel, queue_name, "example_basic_pub_sub").await;


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