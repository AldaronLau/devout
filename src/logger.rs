use core::fmt;

use crate::{LogLevel, Target};

/// A struct providing info about a log message.
#[non_exhaustive]
#[derive(Debug)]
pub struct Log<'a> {
    pub(crate) level: LogLevel,
    pub(crate) target: &'a Target,
    pub(crate) args: fmt::Arguments<'a>,
}

/// Default logger
pub fn logger(log: &Log<'_>) {
    #[cfg(feature = "std")]
    {
        use std::{io::Write, time::SystemTime};

        use yansi::Paint;

        let mut handle = STD_ERR.lock();
        let args = &log.args;
        let log_level = match &log.level {
            LogLevel::Fatal => 'F'.red().bold(),
            LogLevel::Critical => 'C'.bright_red().bold(),
            LogLevel::Error => 'E'.magenta().bold(),
            LogLevel::Warn => 'W'.cyan().bold(),
            LogLevel::Notice => 'N'.green().bold(),
            LogLevel::Info => 'I'.bright_green().bold(),
            LogLevel::Debug => 'D'.yellow().bold(),
            LogLevel::Trace => 'T'.bright_black().bold(),
        };
        let name = log.target.name.bright_yellow().bold();
        let open = '['.bright_white().bold();
        let close = ']'.bright_white().bold();
        let time = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap_or_default()
            .as_micros();
        let time = time.white();

        handle
            .write_fmt(format_args!(
                "{open}{time:X} {log_level} {name}{close} {args}\n",
            ))
            .ok();
    }
}

#[cfg(feature = "std")]
use std::{
    io::{self, Stderr},
    sync::LazyLock,
};

#[cfg(feature = "std")]
static STD_ERR: LazyLock<Stderr> = LazyLock::new(io::stderr);
