mod proto {
    include!("../echo_unary.rs");
}
use proto::{echo_client::EchoClient, EchoRequest};

#[tokio::main]
async fn main() {
    println!("rust grpc unary client");

    let mut client = EchoClient::connect("http://[::1]:8000").await.unwrap();

    let request = tonic::Request::new(EchoRequest {
        data: "openmind3d".to_string(),
    });

    let response = client.echo(request).await.unwrap();
    println!("Response: {}", response.get_ref().data);
}
