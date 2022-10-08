mod proto {
    include!("../echo_unary.rs");
}
use proto::{
    echo_server::{Echo, EchoServer},
    EchoRequest, EchoResponse,
};

use tonic::{transport::Server, Request, Response, Status};

struct MyEchoServer {}

#[tonic::async_trait]
impl Echo for MyEchoServer {
    async fn echo(&self, request: Request<EchoRequest>) -> Result<Response<EchoResponse>, Status> {
        Ok(Response::new(EchoResponse {
            data: format!("hello {}", request.get_ref().data),
        }))
    }
}

#[tokio::main]
async fn main() {
    let echo_server = MyEchoServer {};
    let addr = "[::1]:8000".parse().unwrap();

    println!("Server listening at: {}", addr);
    Server::builder()
        .add_service(EchoServer::new(echo_server))
        .serve(addr)
        .await
        .unwrap();
}
