use colored::*;
use std::io::{self, Write};

pub struct CliPresenter;

impl CliPresenter {
    pub fn new() -> Self {
        Self
    }

    pub fn confirm_task_execution(&self, task: &str) -> io::Result<bool> {
        print!("Execute task '{}' ? (y/N): ", task.red());
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        Ok(input.trim().to_lowercase() == "y")
    }

    pub fn show_success(&self, task: &str) {
        println!(
            "{}",
            format!("Successfully executed task: {}", task).green()
        );
    }

    pub fn show_cancelled(&self) {
        println!("{}", "Task execution cancelled.".yellow());
    }

    pub fn show_no_selection(&self) {
        println!("{}", "No task selected.".yellow());
    }
}
