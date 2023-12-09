use tonic::{Request, Response, Status};

use hello_world::greeter_server::{Greeter};
use hello_world::{HelloReply, HelloRequest};

pub mod hello_world {
    tonic::include_proto!("hello_world");
}

#[derive(Default)]
pub struct MyGreeter {}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        println!("Got a request from {:?}", request.remote_addr());

        let reply = HelloReply {
            message: format!("Hello {}!", request.into_inner().name),
        };
        Ok(Response::new(reply))
    }
}
