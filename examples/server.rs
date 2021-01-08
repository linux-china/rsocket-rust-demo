use log::{info};
use env_logger::Env;

use rsocket_rust::prelude::*;
use rsocket_rust_transport_tcp::TcpServerTransport;
use std::env;
use std::error::Error;
use anyhow::Result;

struct RSocketHandler {
    rsocket: Box<dyn RSocket>
}

impl RSocket for RSocketHandler {
    fn metadata_push(&self, _req: Payload) -> Mono<()> {
        Box::pin(async {})
    }
    fn fire_and_forget(&self, req: Payload) -> Mono<()> {
        info!("{:?}", req);
        Box::pin(async {})
    }

    fn request_response(&self, req: Payload) -> Mono<Result<Payload>> {
        info!("{:?}", req);
        Box::pin(async move { Ok(req) })
    }

    fn request_stream(&self, _req: Payload) -> Flux<Result<Payload>> {
        Box::pin(futures::stream::empty())
    }

    fn request_channel(&self, _reqs: Flux<Result<Payload>>) -> Flux<Result<Payload>> {
        Box::pin(futures::stream::empty())
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let addr = env::args().nth(1).unwrap_or("127.0.0.1:7878".to_string());
    RSocketFactory::receive()
        .transport(TcpServerTransport::from(addr))
        .acceptor(Box::new(|setup, rsocket| {
            info!("accept setup: {:?}", setup);
            Ok(Box::new(RSocketHandler { rsocket }))
            // Or you can reject setup
            // Err(From::from("SETUP_NOT_ALLOW"))
        }))
        .on_start(Box::new(|| info!("+++++++ echo server started! +++++++")))
        .serve()
        .await
}

