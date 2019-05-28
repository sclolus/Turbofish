use log::{Level, Metadata, Record};
use log::{LevelFilter, SetLoggerError};

use super::ansi_escape_code::color::Colored;

struct SimpleLogger;

impl log::Log for SimpleLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let level_str = match record.level() {
                Level::Info => "INFO".green(),
                Level::Trace => "TRACE".white(),
                Level::Error => "ERROR".red(),
                Level::Warn => "WARN".yellow(),
                Level::Debug => "DEBUG".cyan(),
            };
            print_syslog!("{} - {}\n", level_str, record.args());
        }
    }

    fn flush(&self) {}
}

static LOGGER: SimpleLogger = SimpleLogger;

pub fn init() -> Result<(), SetLoggerError> {
    log::set_logger(&LOGGER).map(|()| log::set_max_level(LevelFilter::Info))
}
