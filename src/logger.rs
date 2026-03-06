use std::sync::OnceLock;

// Re-export types from hyprslog
pub use hyprs_log::{CleanupOptions, CleanupResult, Level, LogStats};

static LOGGER: OnceLock<hyprs_log::Logger> = OnceLock::new();

/// Get the global logger instance, initialized with default settings for hyprsink.
fn get_logger() -> &'static hyprs_log::Logger {
    LOGGER.get_or_init(|| {
        hyprs_log::Logger::builder()
            .level(Level::Debug)
            .terminal()
            .colors(true)
            .done()
            .file()
            .app_name("hyprsink")
            .done()
            .json()
            .app_name("hyprsink")
            .done()
            .build()
    })
}

/// Log a message with the given level and scope.
pub fn log(level: Level, scope: &str, msg: &str) {
    get_logger().log(level, scope, msg);
}

/// Log an info message.
pub fn info(scope: &str, msg: &str) {
    get_logger().info(scope, msg);
}

/// Log a debug message.
pub fn debug(scope: &str, msg: &str) {
    get_logger().debug(scope, msg);
}

/// Log a warning message.
pub fn warn(scope: &str, msg: &str) {
    get_logger().warn(scope, msg);
}

/// Log an error message.
pub fn error(scope: &str, msg: &str) {
    get_logger().error(scope, msg);
}

/// Log a trace message.
pub fn trace(scope: &str, msg: &str) {
    get_logger().trace(scope, msg);
}
