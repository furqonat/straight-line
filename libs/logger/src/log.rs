use crate::logger::LogLevel;
use crate::logger::Logger;

#[derive(Debug, Default)]
pub struct Log;

impl Logger for Log {
    #[cfg(debug_assertions)]
    fn info(&self, tag: &str, message: &str) {
        println!("{}: {}, {}", LogLevel::Info, tag, message);
    }

    #[cfg(debug_assertions)]
    fn error(&self, tag: &str, message: &str) {
        println!("{}: {}, {}", LogLevel::Error, tag, message);
    }

    #[cfg(debug_assertions)]
    fn warn(&self, tag: &str, message: &str) {
        println!("{}: {}, {}", LogLevel::Warn, tag, message);
    }

    #[cfg(debug_assertions)]
    fn debug(&self, tag: &str, message: &str) {
        println!("{}: {}, {}", LogLevel::Debug, tag, message);
    }

    #[cfg(not(debug_assertions))]
    fn info(&self, _tag: &str, _message: &str) {
        // Do nothing or implement a release logging mechanism
    }

    #[cfg(not(debug_assertions))]
    fn error(&self, _tag: &str, _message: &str) {
        // Do nothing or implement a release logging mechanism
    }

    #[cfg(not(debug_assertions))]
    fn warn(&self, _tag: &str, _message: &str) {
        // Do nothing or implement a release logging mechanism
    }

    #[cfg(not(debug_assertions))]
    fn debug(&self, _tag: &str, _message: &str) {
        // Do nothing or implement a release logging mechanism
    }
}
