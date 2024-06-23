use sqlx::PgPool;
use tokio::net::TcpListener;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use zero2prod::startup::build_server;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    init_tracing();
    let settings =
        zero2prod::configuration::get_configuration().expect("Failed to read configuration.");
    let pg_pool = PgPool::connect(&settings.database.connection_string())
        .await
        .expect("Failed to connect to Postgres.");
    let address = format!("127.0.0.1:{}", settings.application_port);
    tracing::info!("Starting server at {}", address);
    let listener = TcpListener::bind(address)
        .await
        .unwrap_or_else(|_| panic!("Failed to bind a port {}", settings.application_port));
    build_server(listener, pg_pool).await
}

fn init_tracing() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                // axum logs rejections from built-in extractors with the `axum::rejection`
                // target, at `TRACE` level. `axum::rejection=trace` enables showing those events
                "zero2prod=debug,tower_http=debug,axum::rejection=trace".into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init()
}
