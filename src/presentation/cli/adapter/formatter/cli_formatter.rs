use colored::*;

pub struct CliFormatter;

impl CliFormatter {
    pub fn new() -> Self {
        Self
    }

    pub fn format_task_name(&self, task: &str) -> String {
        task.red().to_string()
    }

    pub fn format_success(&self, message: &str) -> String {
        message.green().to_string()
    }

    pub fn format_warning(&self, message: &str) -> String {
        message.yellow().to_string()
    }
}
