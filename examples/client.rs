use rsocket_rust::prelude::*;
use rsocket_rust_transport_tcp::TcpClientTransport;

#[tokio::main]
async fn main() {
    let client = RSocketFactory::connect()
        .acceptor(Box::new(|| Box::new(EchoRSocket)))
        .transport(TcpClientTransport::from("127.0.0.1:7878"))
        .setup(Payload::from("READY!"))
        .mime_type("text/plain", "text/plain")
        .start()
        .await
        .unwrap();
    let req = Payload::builder()
        .set_data_utf8("Hello World!")
        .set_metadata_utf8("Rust")
        .build();
    let res = client.request_response(req).await.unwrap();
    println!("got: {:?}", res);
    client.close();
}
