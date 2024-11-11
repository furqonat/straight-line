use colored::*;
use std::fmt::{self, Display};

#[allow(dead_code)]
pub enum LogLevel {
    Info,
    Error,
    Warn,
    Debug,
}

impl Display for LogLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let level_str = match self {
            LogLevel::Info => "INFO".green(),
            LogLevel::Error => "ERROR".red(),
            LogLevel::Warn => "WARN".yellow(),
            LogLevel::Debug => "DEBUG".blue(),
        };
        write!(f, "{}", level_str)
    }
}

#[allow(dead_code)]
pub trait Logger {
    fn info(&self, level: &LogLevel, message: &str);
    fn error(&self, level: &LogLevel, message: &str);
    fn warn(&self, level: &LogLevel, message: &str);
    fn debug(&self, level: &LogLevel, message: &str);
}

pub struct LoggerConfig {
    pub save_to_file: bool,
    pub path: Option<String>,
}
