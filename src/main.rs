use sqlx::PgPool;
use tokio::net::TcpListener;
use zero2prod::{
    startup::build_server,
    telemetry::{get_subscriber, init_subscriber},
};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let settings =
        zero2prod::configuration::get_configuration().expect("Failed to read configuration.");
    let pg_pool = PgPool::connect_lazy(&settings.database.connection_string())
        .expect("Failed to connect to Postgres.");
    let address = format!("127.0.0.1:{}", settings.application_port);
    tracing::info!("Starting server at {}", address);
    let listener = TcpListener::bind(address)
        .await
        .unwrap_or_else(|_| panic!("Failed to bind a port {}", settings.application_port));
    build_server(listener, pg_pool).await
}
