use tonic::{transport::Server, Request, Response, Status};

use referral::greeter_server::{Greeter, GreeterServer};
use referral::{HelloReply, HelloRequest};

pub mod referral {
    tonic::include_proto!("referral");
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

        let reply = referral::HelloReply {
            message: format!("Hello {}!", request.into_inner().name),
        };
        Ok(Response::new(reply))
    }
}
