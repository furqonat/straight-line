use crate::logger::{Logger, LoggerConfig};

pub struct Log {
    pub config: Option<LoggerConfig>,
}

#[allow(dead_code)]
impl Log {
    fn new(config: Option<LoggerConfig>) -> Self {
        Self { config }
    }
}

impl Logger for Log {
    #[cfg(debug_assertions)]
    fn info(&self, level: &crate::logger::LogLevel, message: &str) {
        println!("{}: {}", level, message);
    }

    fn error(&self, level: &crate::logger::LogLevel, message: &str) {
        print!("{}: {}", level, message);
    }

    fn warn(&self, level: &crate::logger::LogLevel, message: &str) {
        print!("{}: {}", level, message);
    }

    fn debug(&self, level: &crate::logger::LogLevel, message: &str) {
        print!("{}: {}", level, message);
    }
}
