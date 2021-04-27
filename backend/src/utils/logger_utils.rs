use std::sync::Arc;

use chrono::{Local, SecondsFormat};
use slog::Drain;
use slog::{o, Logger, Never, SendSyncRefUnwindSafeDrain};
use slog::{FnValue, PushFnValue};
use slog_json::Json;
use slog_term::{CompactFormat, TermDecorator};

use crate::common::logger::StructuredLogger;
use crate::config::{AppEnvironment, CONFIG};

pub fn init_logger() -> Logger<Arc<dyn SendSyncRefUnwindSafeDrain<Ok = (), Err = Never>>> {
    let drain = match &CONFIG.environment {
        AppEnvironment::Production => {
            let drain = Json::new(std::io::stdout())
                .add_key_value(o!(
                "@timestamp" => PushFnValue(move |_, ser| {
                    ser.emit(Local::now().to_rfc3339_opts(SecondsFormat::Secs, true))
                }),
                "loglevel" => FnValue(move |rinfo| {
                    rinfo.level().as_str()
                }),
                "msg" => PushFnValue(move |record, ser| {
                    ser.emit(record.msg())
                }),
                ))
                .build()
                .fuse();

            // although not strictly required, using slog_async is always a good idea to unblock the main-thread
            // otherwise it would block until logging has been completed
            slog_async::Async::new(drain).build().fuse()
        }
        AppEnvironment::Development => {
            // TermDecorator with CompactFormat is probably nicer for (local) development
            let decorator = TermDecorator::new().build();
            let drain = CompactFormat::new(decorator).build().fuse();

            slog_async::Async::new(drain).build().fuse()
        }
    };

    slog::Logger::root(
        drain,
        o!("version" => env!("CARGO_PKG_VERSION"), "service" => &CONFIG.name, "log_type" => "application", "application_type" => "service", "module" => FnValue(move |info| {
            info.module().to_string()
        })),
    )
}

pub fn build_logger(root_logger: &slog::Logger) -> StructuredLogger {
    StructuredLogger::new(
        root_logger
            .new(o!("log_type" => "access", "protocol" => "http", "server_name" => &CONFIG.name)),
    )
    .exclude("/liveness")
}
