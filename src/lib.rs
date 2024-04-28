use tracing::Level;
use tracing_appender::non_blocking::{NonBlocking, WorkerGuard};
use tracing_appender::rolling::Rotation;
use tracing_subscriber::fmt::writer::{MakeWriterExt, WithMaxLevel};
use tracing_subscriber::fmt::{Layer, Subscriber};
use tracing_subscriber::layer::SubscriberExt;
pub mod prelude {
    pub use crate::{get_config, init_tracing};
    pub use tracing::{debug, error, info, instrument, trace, warn};
    pub use twba_backup_config::{self, Conf};
    pub use twba_local_db;
}
use prelude::*;

pub fn get_config() -> Conf {
    twba_backup_config::get_default_builder()
        .load()
        .expect("Failed to load config")
}

pub fn init_tracing(crate_name: &str) -> Vec<WorkerGuard> {
    let (guard1, warn_file) = file_tracer(crate_name, Level::WARN, Rotation::DAILY);
    let (guard2, info_file) = file_tracer(crate_name, Level::INFO, Rotation::DAILY);
    let (guard3, trace_file) = file_tracer(crate_name, Level::TRACE, Rotation::DAILY);

    let file_subscriber = Subscriber::builder()
        .with_env_filter(format!("warn,{}=trace", crate_name))
        .finish()
        .with(Layer::default().with_writer(warn_file).json())
        .with(Layer::default().with_writer(info_file).json())
        .with(Layer::default().with_writer(trace_file).json());

    // Set the layered subscriber as the global default
    tracing::subscriber::set_global_default(file_subscriber)
        .expect("Failed to set global default subscriber");
    info!("Tracing initialized for {}", crate_name);
    vec![guard1, guard2, guard3]
}

pub fn file_tracer(
    crate_name: &str,
    level: Level,
    rotation: Rotation,
) -> (WorkerGuard, WithMaxLevel<NonBlocking>) {
    let dir = get_config().log_path();
    let trace_writer = tracing_appender::rolling::RollingFileAppender::builder()
        .rotation(rotation)
        .filename_prefix(crate_name)
        .filename_suffix(format!("{}.{}", level, "log"))
        .build(dir)
        .unwrap();
    // let trace_writer = trace_writer.with_max_level(Level::TRACE);
    let (file, guard) = tracing_appender::non_blocking(trace_writer);
    let file = file.with_max_level(level);
    (guard, file)
}
