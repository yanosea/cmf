use crate::usecase::constant::*;
use std::process::Command;

pub fn execute_task(task: &str) -> Result<(), Box<dyn std::error::Error>> {
    Command::new(COMMAND_CARGO)
        .arg(COMMAND_MAKE)
        .arg(task)
        .status()
        .map_err(|_| format!("{}: {}", ERROR_FAILED_TO_EXECUTE_TASK, task))?;
    Ok(())
}
