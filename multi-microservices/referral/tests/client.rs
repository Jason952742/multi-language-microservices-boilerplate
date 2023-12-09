use std::time::Duration;
use tower::timeout::Timeout;
use hello_world::greeter_client::GreeterClient;
use hello_world::HelloRequest;
use tonic::transport::Channel;

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

    say_hi("Bob".into()).await?;

    Ok(())
}

#[tracing::instrument]
async fn say_hi(name: String) -> Result<(), Box<dyn std::error::Error>> {
    let channel = Channel::from_static("http://127.0.0.1:50051").connect().await?;

    // test timeout
    let timeout_channel = Timeout::new(channel, Duration::from_millis(1000));

    let mut client = GreeterClient::new(timeout_channel);

    let request = tonic::Request::new(HelloRequest {
        name: "Tonic".into(),
    });

    tracing::info!(
        message = "Sending request.",
        request = %request.get_ref().name
    );

    let response = client.say_hello(request).await?;

    tracing::info!(
        message = "Got a response.",
        response = %response.get_ref().message
    );

    Ok(())
}