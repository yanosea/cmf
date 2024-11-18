use crate::domain::model::task::Task;
use crate::presentation::handler::constant::*;
use crate::usecase::{
    execute_task::execute_task, get_tasks::get_tasks_from_makefile,
    select_task::select_task_with_fzf,
};
use colored::*;
use std::env;
use std::io::{self, Write};
use std::process::exit;

// run is the function to run the CLI handler.
pub fn run() {
    // get arguments from the command line
    let args: Vec<String> = env::args().skip(1).collect();
    // get tasks from the Makefile.toml
    match get_tasks_from_makefile() {
        // if tasks are successfully retrieved, handle the tasks
        Ok(tasks) => handle_tasks(tasks, &args),
        // if tasks are not successfully retrieved, handle the error
        Err(e) => handle_error(ERROR_FAILED_TO_GET_TASKS, e),
    }
}

// handle_tasks is the function to handle the tasks.
fn handle_tasks(tasks: Vec<Task>, args: &[String]) {
    // get the names of the tasks
    let task_names: Vec<String> = tasks.iter().map(|task| task.name.clone()).collect();
    // select a task with fzf
    match select_task_with_fzf(&task_names, args) {
        // if a task is successfully selected, confirm and execute the task
        Ok(Some(selected_task)) => confirm_and_execute_task(selected_task),
        // if no task is selected, print a message and exit with 0
        Ok(None) => {
            println!("{}", MESSAGE_NO_TASK_SELECTED.yellow());
            exit(0);
        }
        // if an error occurs, handle the error
        Err(e) => handle_error(ERROR_FAILED_TO_SELECT_TASK, e),
    }
}

// confirm_and_execute_task is the function to confirm and execute the task.
fn confirm_and_execute_task(selected_task: String) {
    // print the prompt to confirm the selected task
    print!(
        "{}",
        PROMPT_SELECTED_TASK.replace("{}", &selected_task.red().to_string())
    );
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    if input.trim().to_lowercase() == CONFIRM_YES {
        // if the input is "yes", print the name of the selected task and execute the task
        println!(
            "{}",
            MESSAGE_SELECTED_TASK
                .replace("{}", &selected_task)
                .blue()
                .underline()
        );
        // execute the task
        if let Err(e) = execute_task(&selected_task) {
            // if an error occurs, handle the error
            handle_error(ERROR_FAILED_TO_EXECUTE_TASK, e);
        }
    } else {
        // if the input is not "yes", print a message and exit with 0
        println!("{}", MESSAGE_NO_TASK_SELECTED.yellow());
        exit(0);
    }
}

// handle_error is the function to handle the error.
fn handle_error(message: &str, error: impl ToString) {
    // print an error message and exit with 1
    eprintln!("{}", format!("{}: {}", message, error.to_string()).red());
    exit(1);
}
