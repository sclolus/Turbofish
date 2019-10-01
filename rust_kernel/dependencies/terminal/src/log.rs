use log::{Level, Metadata, Record};
use log::{LevelFilter, SetLoggerError};

use ansi_escape_code::color::Colored;

pub struct SimpleLogger {
    /// External function binding: Usefull if somebody else want the log !
    binded_fn: Option<fn(&Record)>,
}

impl log::Log for SimpleLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            if let Some(function) = self.binded_fn {
                (function)(record);
            }
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

impl SimpleLogger {
    /// Const fn.
    const fn new() -> Self {
        Self { binded_fn: None }
    }

    /// Bind an external function
    pub fn bind(&mut self, binded_function: fn(&Record)) {
        self.binded_fn = Some(binded_function);
    }

    /// Unbind external function
    pub fn unbind(&mut self) {
        self.binded_fn = None;
    }
}

pub static mut LOGGER: SimpleLogger = SimpleLogger::new();

pub fn init() -> Result<(), SetLoggerError> {
    unsafe { log::set_logger(&LOGGER).map(|()| log::set_max_level(LevelFilter::Info)) }
}
