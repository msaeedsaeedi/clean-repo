mod cli;
mod cleaner;
mod git;
mod patterns;

use anyhow::Result;
use std::process;

use cli::Cli;
use cleaner::Cleaner;

// Exit codes matching the bash script
const EXIT_SUCCESS: i32 = 0;
const EXIT_INVALID_ARGS: i32 = 1;
const EXIT_NO_GITIGNORE: i32 = 2;
const EXIT_NOT_GIT_REPO: i32 = 3;
const EXIT_OPERATION_FAILED: i32 = 4;

fn run() -> Result<i32> {
    let cli = Cli::parse_args();

    // Create and run the cleaner
    let cleaner = Cleaner::new(&cli)?;
    let result = cleaner.run()?;

    // Return appropriate exit code
    if !result.failed.is_empty() {
        Ok(EXIT_OPERATION_FAILED)
    } else {
        Ok(EXIT_SUCCESS)
    }
}

fn main() {
    let exit_code = match run() {
        Ok(code) => code,
        Err(e) => {
            // Determine exit code based on error message
            let error_msg = e.to_string();
            let exit_code = if error_msg.contains("Not inside a Git repository") {
                EXIT_NOT_GIT_REPO
            } else if error_msg.contains(".gitignore not found") {
                EXIT_NO_GITIGNORE
            } else if error_msg.contains("Invalid glob pattern") {
                EXIT_INVALID_ARGS
            } else {
                EXIT_OPERATION_FAILED
            };

            eprintln!("✗ {}", error_msg);
            exit_code
        }
    };

    process::exit(exit_code);
}
