use crate::presentation::cli::adapter::formatter::CliFormatter;
use std::io::{self, Write};

pub struct CliPresenter {
    formatter: CliFormatter,
}

impl CliPresenter {
    pub fn new() -> Self {
        Self {
            formatter: CliFormatter::new(),
        }
    }

    pub fn confirm_task_execution(&self, task: &str) -> io::Result<bool> {
        print!("Execute task '{}' ? (y/N): ", task);
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        Ok(input.trim().to_lowercase() == "y")
    }

    pub fn show_success(&self, task: &str) {
        println!(
            "{}",
            self.formatter
                .format_success(&format!("Successfully executed task: {}", task))
        );
    }

    pub fn show_cancelled(&self) {
        println!(
            "{}",
            self.formatter.format_warning("Task execution cancelled.")
        );
    }

    pub fn show_no_selection(&self) {
        println!("{}", self.formatter.format_warning("No task selected."));
    }
}
