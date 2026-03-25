use std::io::Read;
use std::process::ExitCode;
pub fn run(event: &str) -> ExitCode {
    let mut input = String::new();
    if let Err(e) = std::io::stdin().read_to_string(&mut input) {
        tracing::warn!(event, error = %e, "failed to read hook stdin");
        return ExitCode::SUCCESS;
    }

    tracing::info!(event, "hook invoked");
    tracing::debug!(event, payload = %input, "hook payload");

    ExitCode::SUCCESS
}
