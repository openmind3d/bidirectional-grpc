mod proto {
    include!("../echo_streaming.rs");
}
use proto::{echo_client::EchoClient, Message};
use tokio_stream::StreamExt;

#[tokio::main]
async fn main() {
    let mut client = EchoClient::connect("http://[::1]:8000").await.unwrap();

    let iter = tokio_stream::iter(1..usize::MAX).map(|i| Message {
        data: format!("msg {}", i),
    });

    let input_stream = iter.take(10);

    let response = client.echo_streaming(input_stream).await.unwrap();

    let mut resp_stream = response.into_inner();

    while let Some(received) = resp_stream.next().await {
        let received = received.unwrap();
        println!("received message {}", received.data);
    }
}
