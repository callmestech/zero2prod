use zero2prod::build_server;

#[tokio::main]
async fn main() -> Result<(), hyper::Error> {
    build_server().await
}
