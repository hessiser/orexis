use anyhow::{anyhow, Context, Result};
use log::{Level, LevelFilter, Metadata, Record};
use slog::{Drain, Logger, o};
use std::{
    fs::OpenOptions,
    sync::{Mutex, OnceLock},
};

static LOGGER: OnceLock<MultiLogger> = OnceLock::new();

pub struct MultiLogger {
    sloggers: Vec<Logger>,
}

impl log::Log for MultiLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= log::max_level()
    }

    fn log(&self, record: &Record) {
        let fmt_log = if let Some(mod_path) = record.module_path() {
            format!("{} {}", mod_path, record.args())
        } else {
            format!("{}", record.args())
        };
        for slogger in &self.sloggers {
            match record.level() {
                Level::Error => slog::error!(slogger, "{}", fmt_log),
                Level::Warn => slog::warn!(slogger, "{}", fmt_log),
                Level::Info => slog::info!(slogger, "{}", fmt_log),
                Level::Debug => slog::debug!(slogger, "{}", fmt_log),
                Level::Trace => slog::trace!(slogger, "{}", fmt_log),
            }
        }
    }

    fn flush(&self) {}
}

impl MultiLogger {
    pub fn init() -> Result<()> {
        let mut sloggers = Vec::new();

        // Info+ log file
        {
            let log_path = concat!(env!("CARGO_PKG_NAME"), ".log");
            let file = OpenOptions::new()
                .create(true)
                .write(true)
                .truncate(true)
                .open(log_path)?;

            let decorator = slog_term::PlainSyncDecorator::new(file);
            let drain = slog_term::FullFormat::new(decorator).build().fuse();
            let drain = Mutex::new(drain).fuse();
            let filtered_drain = slog::LevelFilter::new(drain, slog::Level::Info).fuse();

            sloggers.push(Logger::root(filtered_drain, o!()));
        }

        // Debug+ log file
        {
            let log_path = concat!(env!("CARGO_PKG_NAME"), ".debug.log");
            let file = OpenOptions::new()
                .create(true)
                .write(true)
                .truncate(true)
                .open(log_path)?;

            let decorator = slog_term::PlainSyncDecorator::new(file);
            let drain = slog_term::FullFormat::new(decorator).build().fuse();
            let drain = Mutex::new(drain).fuse();
            let filtered_drain = slog::LevelFilter::new(drain, slog::Level::Debug).fuse();

            sloggers.push(Logger::root(filtered_drain, o!()));
        }

        // Terminal
        {
            let decorator = slog_term::TermDecorator::new().build();
            let drain = slog_term::FullFormat::new(decorator).build().fuse();
            let drain = Mutex::new(drain).fuse();

            let level = if cfg!(debug_assertions) {
                slog::Level::Debug
            } else {
                slog::Level::Info
            };

            let filtered_drain = slog::LevelFilter::new(drain, level).fuse();

            sloggers.push(Logger::root(filtered_drain, o!()));
        }

        let multi_logger = MultiLogger {
            sloggers,
        };

        if LOGGER.set(multi_logger).is_err() {
            return Err(anyhow!("Failed to initialize MultiLogger"));
        }
        
        if log::set_logger(LOGGER.get().context("Failed to get MultiLogger")?).is_err() {
            return Err(anyhow!("Failed to set MultiLogger"));
        }
        log::set_max_level(LevelFilter::Trace);
        Ok(())
    }
}