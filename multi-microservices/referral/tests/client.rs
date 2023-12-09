use hello_world::greeter_client::GreeterClient;
use hello_world::HelloRequest;

pub mod hello_world {
    tonic::include_proto!("hello_world");
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
    let mut client = GreeterClient::connect("http://127.0.0.1:50051").await?;

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