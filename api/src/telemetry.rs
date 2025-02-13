use opentelemetry::trace::TracerProvider;
use opentelemetry::KeyValue;
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::Resource;
use tracing::subscriber::set_global_default;
use tracing_log::LogTracer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::{fmt, EnvFilter, Registry};

pub fn initialize_tracing_subscribe(env_filter: String, name: String, address: Option<String>) {
    let env_filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(env_filter));

    let span_builder = opentelemetry_otlp::SpanExporter::builder().with_tonic();
    let span_builder = match address {
        Some(val) => span_builder.with_endpoint(val),
        None => span_builder,
    };

    let tracer = opentelemetry_sdk::trace::TracerProvider::builder()
        .with_batch_exporter(
            span_builder
                .build()
                .expect("Failed to build the opentelemetry tracer"),
            opentelemetry_sdk::runtime::Tokio,
        )
        .with_resource(Resource::new(vec![KeyValue::new("service.name", name)]))
        .build()
        .tracer("jaeger-tracing");

    let telemetry = tracing_opentelemetry::layer().with_tracer(tracer);

    let subscriber = Registry::default()
        .with(env_filter)
        .with(telemetry)
        .with(fmt::layer().pretty());

    LogTracer::init().expect("Failed to set logger");
    set_global_default(subscriber).expect("Failed to set subscriber");
}
