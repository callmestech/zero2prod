use sqlx::postgres::PgPoolOptions;
use tokio::net::TcpListener;
use zero2prod::{
    configuration,
    startup::build_server,
    telemetry::{get_subscriber, init_subscriber},
};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let settings = configuration::get_configuration().expect("Failed to read configuration.");
    let pg_pool = PgPoolOptions::new().connect_lazy_with(settings.database.with_db());
    let address = format!(
        "{}:{}",
        &settings.application.host, settings.application.port
    );
    tracing::info!("Starting server at {}", address);
    tracing::info!("Configuration: {:#?}", settings);

    let listener = TcpListener::bind(address)
        .await
        .unwrap_or_else(|_| panic!("Failed to bind a port {}", settings.application.port));
    build_server(listener, pg_pool).await
}
