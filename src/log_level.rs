/// Log level
///
/// These log levels match the Daku log levels.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
#[repr(u8)]
pub enum LogLevel {
    /// Something is completely broken / unusable
    Fatal = 1,
    /// Something that may cause severe issues happened
    Critical = 2,
    /// Something didn't work as expected
    Error = 3,
    /// Something might not work as expected
    Warn = 4,
    /// A significant event has occured
    Notice = 5,
    /// Some useful information was collected
    Info = 6,
    /// Some low priority information was collected
    Debug = 7,
    /// Something happened (verbose)
    Trace = 8,
}

impl LogLevel {
    pub(crate) fn new(level: u8) -> Option<Self> {
        Some(match level {
            1 => Self::Fatal,
            2 => Self::Critical,
            3 => Self::Error,
            4 => Self::Warn,
            5 => Self::Notice,
            6 => Self::Info,
            7 => Self::Debug,
            8 => Self::Trace,
            _ => return None,
        })
    }
}
