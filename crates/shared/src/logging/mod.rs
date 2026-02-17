pub mod models;

use crate::logging::models::{LogConfig, LogFormat, LogRotation};
use std::path::Path;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_appender::rolling::{Rotation, RollingFileAppender};
use tracing_subscriber::fmt::format::FmtSpan;
use tracing_subscriber::prelude::*;
use tracing_subscriber::{fmt, Registry};

pub fn init_logging(config: &LogConfig) -> Option<WorkerGuard> {
    let mut guard = None;

    let level: tracing::Level = config.level.into();

    let stdout_layer = fmt::layer()
        .with_target(true)
        .with_line_number(true)
        .with_thread_ids(true)
        .with_span_events(FmtSpan::CLOSE);

    let stdout_layer = match config.format {
        LogFormat::Json => stdout_layer.json().boxed(),
        LogFormat::Text => stdout_layer.boxed(),
    };

    let registry = Registry::default()
        .with(stdout_layer.with_filter(tracing_subscriber::filter::LevelFilter::from_level(level)));

    if let Some(ref path_str) = config.file_path {
        let path = Path::new(path_str);
        let directory = path.parent().unwrap_or_else(|| Path::new("."));
        let file_name = path
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("app.log");

        let rotation = match config.rotation {
            LogRotation::Hourly => Rotation::HOURLY,
            LogRotation::Daily => Rotation::DAILY,
            LogRotation::Never => Rotation::NEVER,
        };

        let file_appender = RollingFileAppender::builder()
            .rotation(rotation)
            .filename_prefix(file_name)
            .max_log_files(config.max_files)
            .build(directory)
            .expect("Failed to initialize file appender");

        let (non_blocking, g) = tracing_appender::non_blocking(file_appender);
        guard = Some(g);

        let file_layer = fmt::layer()
            .with_writer(non_blocking)
            .with_ansi(false)
            .with_target(true)
            .with_line_number(true)
            .with_thread_ids(true)
            .with_span_events(FmtSpan::CLOSE);

        let file_layer = match config.format {
            LogFormat::Json => file_layer.json().boxed(),
            LogFormat::Text => file_layer.boxed(),
        };

        let registry = registry.with(file_layer.with_filter(tracing_subscriber::filter::LevelFilter::from_level(level)));
        registry.init();
    } else {
        registry.init();
    }

    guard
}
