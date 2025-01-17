use std::io::Sink;
use tracing::Subscriber;
use tracing::subscriber::set_global_default;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::{EnvFilter, Registry};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::fmt::MakeWriter;

/// Compose multiple layers into a tracings subscriber.
///
/// # Implementation Notes
///
/// We are using impl Subscriber as a return type to avoid having to spell out the actual type
/// returned subscriber, which is indeed quite complex.
/// We need to explicitly call out that the returned subscriber is Send and Sync to make it possible
/// to pass it to init_subscriber later on
pub fn get_subscriber<Sink>(
    name: String,
    env_filter: String,
    sink: Sink,
) -> impl Subscriber + Send + Sync
    where
        // this weird syntax is higher-ranked trait bound (HRTB).  It basically means that Sink implements
        // the MakeWriter trait for all choices of the lifetime parameter 'a'
        // check out https://doc.rust-lang.org/nomicon/hrtb.html for more details
        Sink: for<'a> MakeWriter<'a> + Send + Sync + 'static
{
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(env_filter));
    let formatting_layer = BunyanFormattingLayer::new(name, sink);

    Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer)
}

/// Register a subscriber as global default to process span data.
///
/// It should only be called once!
pub fn init_subscriber(subscriber: impl Subscriber + Send + Sync) {
    // redirect all 'log's events to our subscriber
    LogTracer::init().expect("Failed to set logger");
    // used by applications to specify what subscriber should be used to process spans.
    set_global_default(subscriber).expect("Failed to set subscriber");
}