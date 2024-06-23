use tracing::{subscriber::set_global_default, Subscriber};
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::{layer::SubscriberExt, EnvFilter};

pub fn get_subscriber(name: String, env_filter: String) -> impl Subscriber + Send + Sync {
    let env_filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(env_filter));
    // let env_filter = tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
    // axum logs rejections from built-in extractors with the `axum::rejection`
    // target, at `TRACE` level. `axum::rejection=trace` enables showing those events
    // "zero2prod=debug,tower_http=debug,axum::rejection=trace".into()
    // });

    let formatting_layer = BunyanFormattingLayer::new(name, std::io::stdout);

    tracing_subscriber::registry()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer)
}

pub fn init_subscriber(subscriber: impl Subscriber + Send + Sync) {
    LogTracer::init().expect("Failed to set logger");
    set_global_default(subscriber).expect("Failed to set subscriber");
}
