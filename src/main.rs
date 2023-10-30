use std::net::TcpListener;

use zero2prod::build_server;

#[tokio::main]
async fn main() -> Result<(), hyper::Error> {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    build_server(listener)?.await
}
