use tonic::{Request, Response, Status};

use hello_world::greeter_server::{Greeter};
use hello_world::{HelloReply, HelloRequest};

pub mod hello_world {
    tonic::include_proto!("helloworld");
}

pub struct MyExtension {
    pub some_piece_of_data: String,
}

#[derive(Debug, Default)]
pub struct MyGreeter {}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    #[tracing::instrument]
    async fn say_hello(&self, request: Request<HelloRequest>) -> Result<Response<HelloReply>, Status> {
        tracing::info!("received request");

        let extension = request.extensions().get::<MyExtension>().unwrap();
        println!("extension data = {}", extension.some_piece_of_data);
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
