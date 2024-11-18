use crate::usecase::constant::*;
use std::process::Command;

// execute_task is a function to execute a task from Makefile.toml with cargo make.
pub fn execute_task(task: &str) -> Result<(), Box<dyn std::error::Error>> {
    // execute a task with cargo make
    Command::new(COMMAND_CARGO)
        .arg(COMMAND_MAKE)
        .arg(task)
        .status()
        .map_err(|_| format!("{}: {}", ERROR_FAILED_TO_EXECUTE_TASK, task))?;
    Ok(())
}
