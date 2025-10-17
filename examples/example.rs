use devout::Target;

const EXAMPLE: Target = devout::target!();
const TARGET: Target = devout::target!("Target");
const HIDDEN: Target = devout::target!("Hidden"); // .into_hidden();

fn main() {
    let var = 4.4;

    devout::trace!(TARGET, "Result: {}", var);
    devout::debug!(EXAMPLE, "Result: {var}");
    devout::info!(TARGET, "Result: {}", 4.4);
    devout::notice!(EXAMPLE, "Result: {}", var);
    devout::warn!(TARGET, "Result: {var}");
    devout::error!(EXAMPLE, "Result: {}", 4.4);
    devout::critical!(HIDDEN, "This won't print");
    devout::fatal!(EXAMPLE, "This will print");

    // Integration with the log crate
    devout::init_log_0_4(&devout::LoggerWrapper(()));
    log_0_4::trace!("Log {}", var);
    log_0_4::debug!("Log {var}");
    log_0_4::info!("Log {}", 4.4);
    log_0_4::warn!(target: "hide", "This won't print");
    log_0_4::error!("This will print");
}
