use std::time::Duration;
use tower::timeout::Timeout;
use hello_world::greeter_client::GreeterClient;
use hello_world::HelloRequest;
use tonic::transport::Channel;
use crate::echo::echo_client::EchoClient;
use crate::echo::EchoRequest;

pub mod hello_world {
    tonic::include_proto!("helloworld");
}

pub mod echo {
    tonic::include_proto!("unaryecho");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::FmtSubscriber::builder()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let channel = Channel::from_static("http://127.0.0.1:50051").connect().await?;

    // test timeout
    let timeout_channel = Timeout::new(channel, Duration::from_millis(1500));

    say_hi(timeout_channel.clone(), "Tonic".into()).await?;
    echo_hello(timeout_channel).await?;

    Ok(())
}

#[tracing::instrument]
async fn say_hi(timeout_channel: Timeout<Channel>, name: String) -> Result<(), Box<dyn std::error::Error>> {
    let mut greeter_client = GreeterClient::new(timeout_channel);

    let request = tonic::Request::new(HelloRequest { name });

    tracing::info!(
        message = "Sending request.",
        request = %request.get_ref().name
    );

    let response = greeter_client.say_hello(request).await?;

    tracing::info!(
        message = "Got a response.",
        response = %response.get_ref().message
    );

    Ok(())
}

#[tracing::instrument]
async fn echo_hello(timeout_channel: Timeout<Channel>) -> Result<(), Box<dyn std::error::Error>> {
    let mut echo_client = EchoClient::new(timeout_channel);

    let request = tonic::Request::new(EchoRequest {
        message: "hello".into(),
    });

    tracing::info!(
        message = "Sending request.",
        request = %request.get_ref().message
    );

    let response = echo_client.unary_echo(request).await?;

    tracing::info!(
        message = "Got a response.",
        response = %response.get_ref().message
    );

    Ok(())
}