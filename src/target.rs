use core::fmt;

use crate::{logger, Log, LogLevel};

/// A configurable log target
#[non_exhaustive]
#[derive(Debug)]
pub struct Target {
    pub(crate) name: &'static str,
    pub(crate) max_level: Option<LogLevel>,
    pub(crate) hidden: bool,
    pub(crate) logger: fn(log: &Log<'_>),
}

impl Target {
    /// Create a new target from name.
    pub const fn new(name: &'static str) -> Self {
        Self {
            name,
            max_level: Some(LogLevel::Trace),
            hidden: false,
            logger,
        }
    }

    /// Write a log at the specified log level.
    ///
    /// Consider using this crate's macros instead to reduce verbosity.
    pub fn log(&self, level: LogLevel, args: fmt::Arguments<'_>) {
        (self.logger)(&Log {
            level,
            target: self,
            args,
        });
    }
}

impl From<&'static str> for Target {
    fn from(name: &'static str) -> Self {
        Self::new(name)
    }
}
