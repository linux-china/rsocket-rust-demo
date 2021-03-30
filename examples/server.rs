use log::{info};
use env_logger::Env;
use rsocket_rust::{Result};

use rsocket_rust::prelude::*;
use rsocket_rust_transport_tcp::TcpServerTransport;
use std::env;
use async_trait::async_trait;

struct RSocketHandler {
    pub rsocket: Box<dyn RSocket>
}

#[async_trait]
impl RSocket for RSocketHandler {
    async fn metadata_push(&self, _req: Payload) -> Result<()> {
        Ok(())
    }

    async fn fire_and_forget(&self, req: Payload) -> Result<()> {
        info!("{:?}", req);
        Ok(())
    }

    async fn request_response(&self, req: Payload) -> Result<Option<Payload>> {
        info!("{:?}", req);
        let payload = Payload::builder().set_data_utf8("Hello").build();
        Ok(Some(payload))
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

