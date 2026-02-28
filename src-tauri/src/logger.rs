use chrono::Local;
use colored::Colorize;
use tracing::{Event, Subscriber};
use tracing::field::{Field, Visit};
use tracing_subscriber::{
    fmt::{self, FormatEvent, FormatFields},
    registry::LookupSpan,
    EnvFilter,
};

pub fn init_logger() {
    colored::control::set_override(true);

    tracing_subscriber::fmt()
        .with_env_filter(
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("velora=debug")))
        .event_format(CustomFormatter)
        .init();
}


pub struct Logger;

impl Logger {
    pub fn info(scope: &str, message: &str) {
        tracing::info!(scope = scope, "{message}");
    }

    pub fn warn(scope: &str, message: &str) {
        tracing::warn!(scope = scope, "{message}");
    }

    pub fn error(scope: &str, message: &str) {
        tracing::error!(scope = scope, "{message}");
    }

    pub fn debug(scope: &str, message: &str) {
        tracing::debug!(scope = scope, "{message}");
    }
}

struct CustomFormatter;

struct VisitorData {
    scope: Option<String>,
    message: Option<String>,
}

impl Visit for VisitorData {
    fn record_str(&mut self, field: &Field, value: &str) {
        match field.name() {
            "scope" => self.scope = Some(value.to_string()),
            "message" => self.message = Some(value.to_string()),
            _ => {}
        }
    }

    fn record_debug(&mut self, field: &Field, value: &dyn std::fmt::Debug) {
        match field.name() {
            "scope" => self.scope = Some(format!("{:?}", value)),
            "message" => self.message = Some(format!("{:?}", value)),
            _ => {}
        }
    }
}

impl<S, N> FormatEvent<S, N> for CustomFormatter
where
    S: Subscriber + for<'a> LookupSpan<'a>,
    N: for<'a> FormatFields<'a> + 'static,
{
    fn format_event(
        &self,
        _ctx: &fmt::FmtContext<'_, S, N>,
        mut writer: fmt::format::Writer<'_>,
        event: &Event<'_>,
    ) -> std::fmt::Result {
        let now = Local::now().format("%Y-%m-%d %H:%M:%S");

        let level = *event.metadata().level();

        let level_colored = match level {
            tracing::Level::INFO => "[INFO]".green(),
            tracing::Level::WARN => "[WARN]".yellow(),
            tracing::Level::DEBUG => "[DEBUG]".blue(),
            tracing::Level::ERROR => "[ERROR]".red(),
            _ => level.to_string().normal(),
        };

        let mut visitor = VisitorData {
            scope: None,
            message: None,
        };

        event.record(&mut visitor);

        let scope = visitor.scope.unwrap_or_else(|| "UNKNOWN".into());
        let message = visitor.message.unwrap_or_default();

        writeln!(
            writer,
            "[{}] [{}] {}: {}",
            now,
            scope,
            level_colored,
            message
        )
    }
}