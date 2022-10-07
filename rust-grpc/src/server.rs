mod proto {
    include!("pingpong.rs");
}
use std::net::ToSocketAddrs;

use proto::{
    ping_pong_service_server::{PingPongService, PingPongServiceServer},
    Message,
};

use tonic::{transport::Server, Request, Response, Status, Streaming};

// #[derive(Debug)]
// pub struct PingPongServer {}

// #[tonic::async_trait]
// impl PingPongService for PingPongServer {
//     async fn ping(&self, req: Request<Message>) -> Result<Self::Message> {}
// }

#[tokio::main]
async fn main() {
    // let server = PingPongServer {};
    // Server::builder()
    //     .add_service(PingPongServiceServer::new(server))
    //     .serve("localhost:8000".to_socket_addrs().unwrap().next().unwrap())
    //     .await
    //     .unwrap();
}
