use std::fs::{self, OpenOptions, File};
use std::io::{Error, ErrorKind};
use std::io::prelude::Write;
use std::ops::Deref;
use std::path::Path;
use std::sync::{Arc, Mutex};

use log::{self, LogRecord, LogLevel, LogMetadata, LogLevelFilter,
          SetLoggerError};
use time;

struct StdoutLogger {
    level: LogLevel,
}

impl log::Log for StdoutLogger {
    fn enabled(&self, metadata: &LogMetadata) -> bool {
        metadata.level() <= self.level
    }

    fn log(&self, record: &LogRecord) {
        if self.enabled(record.metadata()) {
            println!("{} - {} - {}",
                     record.level(),
                     time::now().to_timespec().sec,
                     record.args());
        }
    }
}

struct FileLogger {
    file: Arc<Mutex<File>>,
    level: LogLevel,
}

impl FileLogger {
    pub fn new(path: &str, level: LogLevel) -> Result<FileLogger, Error> {
        let ospath = Path::new(path).parent();
        if ospath.is_none() {
            return Err(Error::new(
                ErrorKind::Other,
                format!("Failed to use log directory: {}", path)
            ));
        }

        match fs::create_dir_all(&ospath.unwrap()) {
            Err(e) => return Err(Error::new(
                ErrorKind::Other,
                format!("Failed to create log directory: {}", e)
            )),
            Ok(_) => (),
        }

        OpenOptions::new()
            .create(true).write(true).append(true).open(path).map( |file| {
                FileLogger{
                    file: Arc::new(Mutex::new(file)),
                    level: level,
                }
            })
    }
}

impl log::Log for FileLogger {
    fn enabled(&self, metadata: &LogMetadata) -> bool {
        metadata.level() <= self.level
    }

    fn log(&self, record: &LogRecord) {
        if self.enabled(record.metadata()) {
            let mut logfile = self.file.clone();
            logfile.lock()
                .unwrap()
                .write_all(format!("{} - {} - {}\n",
                                   record.level(),
                                   time::now().to_timespec().sec,
                                   record.args()).as_bytes());
        }
    }
}

pub fn init_logger(path: Option<String>, level: LogLevel) -> Result<(), SetLoggerError> {
    let logger: Box<log::Log> = match path {
        Some(p) => Box::new(FileLogger::new(p.trim_left(), level).unwrap()),
        None => Box::new(StdoutLogger{ level: level })
    };

    log::set_logger(|max_log_level| {
        max_log_level.set(LogLevelFilter::Debug);
        logger
    })
}
