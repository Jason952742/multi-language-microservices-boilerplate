use std::time::Duration;
use tokio::time::sleep;
use tonic::{Request, Response, Status};

use hello_world::greeter_server::{Greeter};
use hello_world::{HelloReply, HelloRequest};

pub mod hello_world {
    tonic::include_proto!("hello_world");
}

#[derive(Debug, Default)]
pub struct MyGreeter {}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    #[tracing::instrument]
    async fn say_hello(&self, request: Request<HelloRequest>) -> Result<Response<HelloReply>, Status> {
        tracing::info!("received request");
        println!("Got a request from {:?}", request.remote_addr());

        // test timeout
        // sleep(Duration::from_millis(5000)).await;

        let reply = HelloReply {
            message: format!("Hello {}!", request.into_inner().name),
        };

        tracing::debug!("sending response");

        Ok(Response::new(reply))
    }
}
