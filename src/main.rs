use colored::*;
use std::env;
use std::process::exit;

mod execution;
mod selection;
mod tasks;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    match tasks::get_tasks_from_makefile() {
        Ok(tasks) => match selection::select_task_with_fzf(&tasks, &args) {
            Ok(Some(Some(selected_task))) => {
                println!(
                    "{}",
                    format!("Selected task : {}", selected_task)
                        .blue()
                        .underline()
                );
                if let Err(e) = execution::execute_task(&selected_task) {
                    eprintln!(
                        "{}",
                        format!("Failed to execute task : {}", e.to_string()).red()
                    );
                    exit(1);
                }
            }
            Ok(Some(None)) => {
                println!("{}", "Task cancelled".yellow());
                exit(0);
            }
            Ok(None) => {
                println!("{}", "No task selected".yellow());
                exit(0);
            }
            Err(e) => {
                eprintln!(
                    "{}",
                    format!("Failed to select task with fzf : {}", e.to_string()).red()
                );
                exit(1);
            }
        },
        Err(e) => {
            eprintln!(
                "{}",
                format!("Failed to get tasks from Makefile : {}", e.to_string()).red()
            );
            exit(1);
        }
    }
}
