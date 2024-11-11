use colored::*;
use std::fmt::{self, Display};

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

pub trait Logger {
    fn info(&self, tag: &str, message: &str);
    fn error(&self, tag: &str, message: &str);
    fn warn(&self, tag: &str, message: &str);
    fn debug(&self, tag: &str, message: &str);
}
