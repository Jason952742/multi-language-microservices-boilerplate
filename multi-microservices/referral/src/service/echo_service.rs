use tonic::{Request, Response, Status};

pub mod echo {
    tonic::include_proto!("unaryecho");
}

use echo::{echo_server::{Echo}, EchoRequest, EchoResponse};

#[derive(Default)]
pub struct MyEcho;

#[tonic::async_trait]
impl Echo for MyEcho {
    async fn unary_echo(
        &self,
        request: Request<EchoRequest>,
    ) -> Result<Response<EchoResponse>, Status> {
        let message = request.into_inner().message;
        Ok(Response::new(EchoResponse { message }))
    }
}
