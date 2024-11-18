use crate::infrastructure::repository::constant::*;
use std::io::Write;
use std::process::{Command, Stdio};

// FzfRepository is a struct to select a task by fzf.
pub struct FzfRepository;

// Implementation of FzfRepository.
impl FzfRepository {
    // new is a constructor of FzfRepository.
    pub fn new() -> Self {
        FzfRepository
    }

    // select_task is a method to select a task by fzf.
    // tasks &Vec<String> : is a reference to a vector of tasks.
    // args  &Vec<String> : is a reference to a vector of arguments.
    // Returns a Result of Option<String> or std::io::Error.
    pub fn select_task(
        &self,
        tasks: &[String],
        args: &[String],
    ) -> Result<Option<String>, std::io::Error> {
        // join tasks
        let tasks = self.join_tasks_with_newline(tasks);
        // join arguments
        let concatenated_args = self.join_args_with_space(args);
        // spawn fzf
        let mut child = self.spawn_fzf(&concatenated_args)?;
        // pass tasks to fzf
        self.pass_tasks_to_fzf(&mut child, &tasks)?;
        // wait for fzf
        let output = self.wait_for_fzf(child)?;
        // get selected task
        self.get_selected_task(output)
    }

    // join_tasks_with_newline is a method to join tasks with a newline.
    // tasks &Vec<String> : is a reference to a vector of tasks.
    // Returns a String.
    fn join_tasks_with_newline(&self, tasks: &[String]) -> String {
        tasks.join("\n")
    }

    // join_args_with_space is a method to join arguments with a space.
    // args &Vec<String> : is a reference to a vector of arguments.
    // Returns a String.
    fn join_args_with_space(&self, args: &[String]) -> String {
        args.join(" ")
    }

    // spawn_fzf is a method to spawn fzf.
    // concatenated_args &str : is a reference to a string of concatenated arguments.
    // Returns a Result of std::process::Child or std::io::Error.
    fn spawn_fzf(&self, concatenated_args: &str) -> Result<std::process::Child, std::io::Error> {
        Command::new(COMMAND_FZF)
            .arg(ARG_QUERY)
            .arg(concatenated_args)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, ERROR_FAILED_TO_SPAWN_FZF))
    }

    // pass_tasks_to_fzf is a method to pass tasks to fzf.
    // child &mut std::process::Child : is a mutable reference to a child process.
    // tasks &str                     : is a reference to a string of tasks.
    // Returns a Result of () or std::io::Error.
    fn pass_tasks_to_fzf(
        &self,
        child: &mut std::process::Child,
        tasks: &str,
    ) -> Result<(), std::io::Error> {
        child
            .stdin
            .as_mut()
            .ok_or_else(|| {
                std::io::Error::new(std::io::ErrorKind::Other, ERROR_FAILED_TO_OPEN_STDIN)
            })?
            .write_all(tasks.as_bytes())
    }

    // wait_for_fzf is a method to wait for fzf.
    // child std::process::Child : is a child process.
    // Returns a Result of std::process::Output or std::io::Error.
    fn wait_for_fzf(
        &self,
        child: std::process::Child,
    ) -> Result<std::process::Output, std::io::Error> {
        child.wait_with_output()
    }

    // get_selected_task is a method to get the selected task.
    // output std::process::Output : is the output of the child process.
    // Returns a Result of Option<String> or std::io::Error.
    fn get_selected_task(
        &self,
        output: std::process::Output,
    ) -> Result<Option<String>, std::io::Error> {
        let selected_task = String::from_utf8(output.stdout)
            .map_err(|_| {
                std::io::Error::new(std::io::ErrorKind::Other, ERROR_FAILED_TO_CONVERT_OUTPUT)
            })?
            .trim()
            .to_string();
        if selected_task.is_empty() {
            Ok(None)
        } else {
            Ok(Some(selected_task))
        }
    }
}
