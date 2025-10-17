/// Create a target with the module path.
#[macro_export]
macro_rules! target {
    () => {
        Target::new(module_path!())
    };
    ($name:literal) => {
        Target::new(concat!(module_path!(), " ", $name))
    };
}

/// Log a message at the trace level.
#[macro_export]
#[clippy::format_args]
macro_rules! trace {
    ($target:expr, $($arg:tt)+) => {
        $crate::Target::from($target).log(
            $crate::LogLevel::Trace,
            format_args!($($arg)+),
        )
    };
}

/// Log a message at the debug level.
#[macro_export]
#[clippy::format_args]
macro_rules! debug {
    ($target:expr, $($arg:tt)+) => {
        $crate::Target::from($target).log(
            $crate::LogLevel::Debug,
            format_args!($($arg)+),
        )
    };
}

/// Log a message at the info level.
#[macro_export]
#[clippy::format_args]
macro_rules! info {
    ($target:expr, $($arg:tt)+) => {
        $crate::Target::from($target).log(
            $crate::LogLevel::Info,
            format_args!($($arg)+),
        )
    };
}

/// Log a message at the notice level.
#[macro_export]
#[clippy::format_args]
macro_rules! notice {
    ($target:expr, $($arg:tt)+) => {
        $crate::Target::from($target).log(
            $crate::LogLevel::Notice,
            format_args!($($arg)+),
        )
    };
}

/// Log a message at the warn level.
#[macro_export]
#[clippy::format_args]
macro_rules! warn {
    ($target:expr, $($arg:tt)+) => {
        $crate::Target::from($target).log(
            $crate::LogLevel::Warn,
            format_args!($($arg)+),
        )
    };
}

/// Log a message at the error level.
#[macro_export]
#[clippy::format_args]
macro_rules! error {
    ($target:expr, $($arg:tt)+) => {
        $crate::Target::from($target).log(
            $crate::LogLevel::Error,
            format_args!($($arg)+),
        )
    };
}

/// Log a message at the critical level.
#[macro_export]
#[clippy::format_args]
macro_rules! critical {
    ($target:expr, $($arg:tt)+) => {
        $crate::Target::from($target).log(
            $crate::LogLevel::Critical,
            format_args!($($arg)+),
        )
    };
}

/// Log a message at the fatal level.
#[macro_export]
#[clippy::format_args]
macro_rules! fatal {
    ($target:expr, $($arg:tt)+) => {
        $crate::Target::from($target).log(
            $crate::LogLevel::Fatal,
            format_args!($($arg)+),
        )
    };
}
