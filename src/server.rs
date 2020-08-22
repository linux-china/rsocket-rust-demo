use log::{info};
use env_logger::Env;

use rsocket_rust::prelude::*;
use rsocket_rust_transport_tcp::TcpServerTransport;
use std::env;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    env_logger::from_env(Env::default().default_filter_or("info")).init();

    let addr = env::args().nth(1).unwrap_or("127.0.0.1:7878".to_string());
    info!("start server");
    RSocketFactory::receive()
        .transport(TcpServerTransport::from(addr))
        .acceptor(Box::new(|setup, _socket| {
            info!("accept setup: {:?}", setup);
            Ok(Box::new(EchoRSocket))
            // Or you can reject setup
            // Err(From::from("SETUP_NOT_ALLOW"))
        }))
        .on_start(Box::new(|| info!("+++++++ echo server started! +++++++")))
        .serve()
        .await
}

