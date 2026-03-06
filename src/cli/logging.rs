//! CLI logging integration with tracing + hyprslog.

use crate::logger;
use anyhow::Result;
use tracing_subscriber::{EnvFilter, prelude::*};

/// Log an info message
pub fn info(scope: &str, msg: &str) {
    logger::info(scope, msg);
    tracing::info!(scope = scope, message = msg);
}

/// Log a debug message
pub fn debug(scope: &str, msg: &str) {
    logger::debug(scope, msg);
    tracing::debug!(scope = scope, message = msg);
}

/// Log a warning message
pub fn warn(scope: &str, msg: &str) {
    logger::warn(scope, msg);
    tracing::warn!(scope = scope, message = msg);
}

/// Log an error message
pub fn error(scope: &str, msg: &str) {
    logger::error(scope, msg);
    tracing::error!(scope = scope, message = msg);
}

pub fn init_logging() -> Result<()> {
    let _ = tracing_log::LogTracer::init();

    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    let registry = tracing_subscriber::registry().with(env_filter);
    let _ = tracing::subscriber::set_global_default(registry);

    Ok(())
}
