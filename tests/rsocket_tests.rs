use rsocket_rust::prelude::*;
use rsocket_rust_transport_tcp::TcpClientTransport;
use rsocket_rust::utils::EchoRSocket;

#[tokio::test]
async fn test_request_response() {
    let client = RSocketFactory::connect()
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
    if let Some(payload) = res {
        println!("got: {}", payload.data_utf8().unwrap());
    }
    println!("{}", "good");
}
