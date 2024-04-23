use tracing::{info, Level};
use tracing_appender::non_blocking::{NonBlocking, WorkerGuard};
use tracing_subscriber::fmt::format::{DefaultFields, Format};
use tracing_subscriber::fmt::{Layer, Subscriber};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::{fmt, EnvFilter};
use twba_backup_config::Conf;

pub fn get_config() -> Conf {
    twba_backup_config::get_default_builder()
        .load()
        .expect("Failed to load config")
}

pub fn init_tracing(crate_name: &str) -> WorkerGuard {
    let (guard, file) = file_tracer(crate_name);

    let file_subscriber = Subscriber::builder()
        .with_max_level(Level::INFO)
        .with_env_filter(format!("warn,{}=trace", crate_name))
        .finish()
        .with(Layer::default().with_writer(file));

    // Set the layered subscriber as the global default
    tracing::subscriber::set_global_default(file_subscriber)
        .expect("Failed to set global default subscriber");
    info!("Tracing initialized for {}", crate_name);
    guard
}

pub fn file_tracer(crate_name: &str) -> (WorkerGuard, NonBlocking) {
    let dir = get_config().log_path();
    let trace_writer = tracing_appender::rolling::RollingFileAppender::builder()
        .rotation(tracing_appender::rolling::Rotation::HOURLY)
        .filename_prefix(format!("{}-trace", crate_name))
        .filename_suffix("log")
        .build(dir)
        .unwrap();
    let (file, guard) = tracing_appender::non_blocking(trace_writer);
    (guard, file)
}
