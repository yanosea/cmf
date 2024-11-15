use std::process::Command;

pub fn execute_task(task: &str) -> Result<(), Box<dyn std::error::Error>> {
    Command::new("cargo")
        .arg("make")
        .arg(task)
        .status()
        .map_err(|_| format!("Failed to execute task: {}", task))?;
    Ok(())
}
