use crate::presentation::handler::constant::*;
use crate::usecase::{
    execute_task::execute_task, get_tasks::get_tasks_from_makefile,
    select_task::select_task_with_fzf,
};
use colored::*;
use std::env;
use std::io::{self, Write};
use std::process::exit;

pub fn run() {
    let args: Vec<String> = env::args().skip(1).collect();
    match get_tasks_from_makefile() {
        Ok(tasks) => {
            let task_names: Vec<String> = tasks.iter().map(|task| task.name.clone()).collect();
            match select_task_with_fzf(&task_names, &args) {
                Ok(Some(selected_task)) => {
                    print!(
                        "{}",
                        PROMPT_SELECTED_TASK.replace("{}", &selected_task.red().to_string())
                    );
                    io::stdout().flush().unwrap();
                    let mut input = String::new();
                    io::stdin().read_line(&mut input).unwrap();
                    if input.trim().to_lowercase() == CONFIRM_YES {
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
                        println!("{}", MESSAGE_NO_TASK_SELECTED.yellow());
                        exit(0);
                    }
                }
                Ok(None) => {
                    println!("{}", MESSAGE_NO_TASK_SELECTED.yellow());
                    exit(0);
                }
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
            eprintln!(
                "{}",
                format!("{}: {}", ERROR_FAILED_TO_GET_TASKS, e.to_string()).red()
            );
            exit(1);
        }
    }
}
