use std::time::Duration;
use tower::timeout::Timeout;
use api::hello_world::greeter_client::GreeterClient;
use api::hello_world::HelloRequest;
use tonic::{
    codegen::InterceptedService,
    service::Interceptor,
    transport::{Channel, Endpoint},
    Request, Status,
};
use tonic::metadata::MetadataValue;
use api::pb::echo_client::EchoClient;
use api::pb::EchoRequest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::FmtSubscriber::builder()
        .with_max_level(tracing::Level::INFO)
        .init();

    let endpoints = ["http://127.0.0.1:50051", "http://127.0.0.1:50052"]
        .iter()
        .map(|a| Channel::from_static(a));

    let channel = Channel::balance_list(endpoints);

    // test timeout
    let timeout_channel = Timeout::new(channel, Duration::from_millis(1500));

    say_hi(timeout_channel.clone(), "Tonic".into()).await?;
    echo_hello(timeout_channel).await?;

    Ok(())
}

#[tracing::instrument]
async fn say_hi(timeout_channel: Timeout<Channel>, name: String) -> Result<(), Box<dyn std::error::Error>> {
    let mut greeter_client = GreeterClient::with_interceptor(timeout_channel, intercept);

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

/// This function will get called on each outbound request. Returning a
/// `Status` here will cancel the request and have that status returned to
/// the caller.
fn intercept(req: Request<()>) -> Result<Request<()>, Status> {
    println!("Intercepting request: {:?}", req);
    Ok(req)
}

// You can also use the `Interceptor` trait to create an interceptor type
// that is easy to name
struct MyInterceptor;

impl Interceptor for MyInterceptor {
    fn call(&mut self, request: tonic::Request<()>) -> Result<tonic::Request<()>, Status> {
        Ok(request)
    }
}

#[allow(dead_code, unused_variables)]
async fn using_named_interceptor() -> Result<(), Box<dyn std::error::Error>> {
    let channel = Endpoint::from_static("http://[::1]:50051")
        .connect()
        .await?;

    let client: GreeterClient<InterceptedService<Channel, MyInterceptor>> =
        GreeterClient::with_interceptor(channel, MyInterceptor);

    Ok(())
}

// Using a function pointer type might also be possible if your interceptor is a
// bare function that doesn't capture any variables
#[allow(dead_code, unused_variables, clippy::type_complexity)]
async fn using_function_pointer_interceptro() -> Result<(), Box<dyn std::error::Error>> {
    let channel = Endpoint::from_static("http://[::1]:50051")
        .connect()
        .await?;

    let client: GreeterClient<
        InterceptedService<Channel, fn(tonic::Request<()>) -> Result<tonic::Request<()>, Status>>,
    > = GreeterClient::with_interceptor(channel, intercept);

    Ok(())
}

#[tracing::instrument]
async fn echo_hello(timeout_channel: Timeout<Channel>) -> Result<(), Box<dyn std::error::Error>> {
    let token: MetadataValue<_> = "Bearer JaXmn2586KvTz".parse()?;

    let mut echo_client = EchoClient::with_interceptor(timeout_channel, move |mut req: Request<()>| {
        req.metadata_mut().insert("authorization", token.clone());
        Ok(req)
    });

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
