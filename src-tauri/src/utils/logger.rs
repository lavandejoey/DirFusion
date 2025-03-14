// /DirFusion/src-tauri/src/utils/logger.rs
// Enable log the log for app in the format:
// [2021-08-07 12:00:00] - [INFO] - [src/main.rs:10] - Hello World
use chrono::Local;
use log::{Level, LevelFilter, Metadata, Record};
use std::fs::OpenOptions;
use std::io::Write;

pub struct DirFusionLogger {
    level: LevelFilter,
}

impl DirFusionLogger {
    pub fn new(level: LevelFilter) -> Self {
        Self { level }
    }
}

impl log::Log for DirFusionLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= self.level
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S");
            let level = record.level();
            let target = record.target();
            let message = record.args();

            println!("[{}] - [{}] - [{}] - {}", timestamp, level, target, message);

            if level == Level::Error {
                if let Ok(mut file) = OpenOptions::new().append(true).open("error.log") {
                    if let Err(e) = writeln!(
                        file,
                        "[{}] - [{}] - [{}] - {}",
                        timestamp, level, target, message
                    ) {
                        eprintln!("Error writing to file: {}", e);
                    }
                } else {
                    eprintln!("Error opening file");
                }
            }
        }
    }

    fn flush(&self) {}
}

pub fn init_logger(level: LevelFilter) -> Result<(), log::SetLoggerError> {
    log::set_max_level(level);
    log::set_boxed_logger(Box::new(DirFusionLogger::new(level)))
}
