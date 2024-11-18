use crate::presentation::handler::constant::*;
use crate::usecase::{
    execute_task::execute_task, get_tasks::get_tasks_from_makefile,
    select_task::select_task_with_fzf,
};
use colored::*;
use std::env;
use std::io::{self, Write};
use std::process::exit;

// run is a function to run the CLI.
pub fn run() {
    // get arguments
    let args: Vec<String> = env::args().skip(1).collect();
    // get tasks from Makefile
    match get_tasks_from_makefile() {
        Ok(tasks) => {
            // get task names
            let task_names: Vec<String> = tasks.iter().map(|task| task.name.clone()).collect();
            // select a task by fzf
            match select_task_with_fzf(&task_names, &args) {
                Ok(Some(selected_task)) => {
                    // if a task is selected, confirm to execute the selected task
                    print!(
                        "{}",
                        PROMPT_SELECTED_TASK.replace("{}", &selected_task.red().to_string())
                    );
                    io::stdout().flush().unwrap();
                    let mut input = String::new();
                    io::stdin().read_line(&mut input).unwrap();
                    if input.trim().to_lowercase() == CONFIRM_YES {
                        // if the input is "yes", execute the selected task
                        println!(
                            "{}",
                            MESSAGE_SELECTED_TASK
                                .replace("{}", &selected_task)
                                .blue()
                                .underline()
                        );
                        if let Err(e) = execute_task(&selected_task) {
                            eprintln!(
                                "{}",
                                format!("{}: {}", ERROR_FAILED_TO_EXECUTE_TASK, e.to_string())
                                    .red()
                            );
                            exit(1);
                        }
                    } else {
                        // if the input is not "yes", exit with 0
                        println!("{}", MESSAGE_NO_TASK_SELECTED.yellow());
                        exit(0);
                    }
                }
                // if no task is selected, exit with 0
                Ok(None) => {
                    println!("{}", MESSAGE_NO_TASK_SELECTED.yellow());
                    exit(0);
                }
                // if failed to select a task, exit with 1
                Err(e) => {
                    eprintln!(
                        "{}",
                        format!("{}: {}", ERROR_FAILED_TO_SELECT_TASK, e.to_string()).red()
                    );
                    exit(1);
                }
            }
        }
        Err(e) => {
            // if failed to get tasks, exit with 1
            eprintln!(
                "{}",
                format!("{}: {}", ERROR_FAILED_TO_GET_TASKS, e.to_string()).red()
            );
            exit(1);
        }
    }
}
