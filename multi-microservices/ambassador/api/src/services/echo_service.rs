use tonic::{Request, Response, Status};
use std::net::SocketAddr;

pub mod pb {
    tonic::include_proto!("unaryecho");
}

use pb::{echo_server::{Echo}, EchoRequest, EchoResponse};

type EchoResult<T> = Result<Response<T>, Status>;

#[derive(Debug)]
pub struct EchoServer {
    pub addr: SocketAddr,
}

#[tonic::async_trait]
impl Echo for EchoServer {
    async fn unary_echo(&self, request: Request<EchoRequest>) -> EchoResult<EchoResponse> {
        let message = format!("{} (from {})", request.into_inner().message, self.addr);

        Ok(Response::new(EchoResponse { message }))
    }
}
