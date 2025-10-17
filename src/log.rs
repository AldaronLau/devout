#[cfg(feature = "log+0.4")]
pub(super) mod version_0_4 {
    ///
    pub struct LoggerWrapper<T>(pub T)
    where
        T: Send + Sync;

    impl<T> log_0_4::Log for LoggerWrapper<T>
    where
        T: Send + Sync,
    {
        fn enabled(&self, metadata: &log_0_4::Metadata<'_>) -> bool {
            true
        }

        fn log(&self, record: &log_0_4::Record<'_>) {
            std::println!("TODO");
        }

        fn flush(&self) {}
    }

    /// Initialize integration with the log crate
    ///
    /// # Panics
    ///
    ///  - If called more than once
    ///  - If a global logger is already initialized
    pub fn init_log_0_4<T>(logger: &'static LoggerWrapper<T>)
    where
        T: Send + Sync,
    {
        log_0_4::set_logger(logger).expect("logger already initialized");
        log_0_4::set_max_level(log_0_4::LevelFilter::Trace);
    }
}
