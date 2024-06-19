use std::net::TcpListener;

use sqlx::PgPool;
use zero2prod::startup::build_server;

#[tokio::main]
async fn main() -> Result<(), hyper::Error> {
    let settings =
        zero2prod::configuration::get_configuration().expect("Failed to read configuration.");
    let pg_pool = PgPool::connect(&settings.database.connection_string())
        .await
        .expect("Failed to connect to Postgres.");
    let address = format!("127.0.0.1:{}", settings.application_port);
    let listener = TcpListener::bind(address)
        .unwrap_or_else(|_| panic!("Failed to bind a port {}", settings.application_port));
    build_server(listener, pg_pool)?.await
}
