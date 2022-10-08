mod proto {
    include!("../echo_streaming.rs");
}
use proto::{
    echo_server::{Echo, EchoServer},
    Message,
};

use futures::Stream;
use std::{error::Error, pin::Pin};
use tokio::sync::mpsc;
use tokio_stream::{wrappers::ReceiverStream, StreamExt};
use tonic::{transport::Server, Request, Response, Status, Streaming};

type EchoResult<T> = Result<Response<T>, Status>;
type ResponseStream = Pin<Box<dyn Stream<Item = Result<Message, Status>> + Send>>;
// type ReceiveStream = Pin<Box<dyn Stream<Item = Result<Message, Status>> + Send>>;

#[derive(Debug)]
struct MyEchoServer {}

#[tonic::async_trait]
impl Echo for MyEchoServer {
    type EchoStreamingStream = ResponseStream;

    async fn echo_streaming(
        &self,
        req: Request<Streaming<Message>>,
    ) -> EchoResult<Self::EchoStreamingStream> {
        let mut input_stream = req.into_inner();
        let (tx, rx) = mpsc::channel(128);

        tokio::spawn(async move {
            while let Some(result) = input_stream.next().await {
                match result {
                    Ok(message) => {
                        println!("Have message {}", message.data);

                        tx.send(Message {
                            data: "some".to_string(),
                        })
                        .await
                        .unwrap();
                    }
                    Err(e) => {}
                }
                // match result {
                //     Ok(value) => tx
                //         .send(Message {
                //             data: "some".to_string(),
                //         })
                //         .await
                //         .expect("working rx"),

                //     Err(err) => {
                //         if let Some(io_err) = match_for_io_error(&err) {
                //             if io_err.kind() == ErrorKind::BrokenPipe {
                //                 eprintln!("\tclient disconnected: broken pipe");
                //                 break;
                //             }
                //         }

                //         match tx.send(Err(err)).await {
                //             Ok(_) => (),
                //             Err(_err) => break,
                //         }
                //     }
                // }
            }
        });

        let out_stream = ReceiverStream::new(rx);
        Ok(Response::new(
            Box::pin(out_stream) as Self::EchoStreamingStream
        ))
    }
}

#[tokio::main]
async fn main() {
    let echo_server = MyEchoServer {};
    let addr: std::net::SocketAddr = "[::1]:8000".parse().unwrap();
    println!("Server listening at: {}", addr);
    Server::builder()
        .add_service(EchoServer::new(echo_server))
        .serve(addr)
        .await
        .unwrap();
}
