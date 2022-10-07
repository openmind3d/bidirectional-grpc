use futures::stream::Stream;
use std::time::Duration;
use tokio_stream::StreamExt;

mod proto {
    include!("pingpong.rs");
}

use proto::{ping_pong_service_client::PingPongServiceClient, Message};

#[tokio::main]
async fn main() {
    let mut client = PingPongServiceClient::connect("localhost:8000")
        .await
        .unwrap();

    // let stream = client
    //     .ping(Message {
    //         Type: "test".into(),
    //     })
    //     .await
    //     .unwrap()
    //     .into_inner();
    // let request = tonic::Request::new(Message::)
}
