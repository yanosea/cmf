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
    pub fn select_task(
        &self,
        tasks: &[String],
        args: &[String],
    ) -> Result<Option<String>, std::io::Error> {
        // join tasks with newline
        let tasks = tasks.join("\n");
        // join args with space
        let concatenated_args = args.join(" ");
        // spawn fzf
        let mut child = Command::new(COMMAND_FZF)
            .arg(ARG_QUERY)
            .arg(&concatenated_args)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .map_err(|_| {
                std::io::Error::new(std::io::ErrorKind::Other, ERROR_FAILED_TO_SPAWN_FZF)
            })?;
        // write tasks to fzf
        child
            .stdin
            .as_mut()
            .ok_or_else(|| {
                std::io::Error::new(std::io::ErrorKind::Other, ERROR_FAILED_TO_OPEN_STDIN)
            })?
            .write_all(tasks.as_bytes())?;
        // wait for fzf
        let output = child.wait_with_output()?;
        // get selected task
        let selected_task = String::from_utf8(output.stdout)
            .map_err(|_| {
                std::io::Error::new(std::io::ErrorKind::Other, ERROR_FAILED_TO_CONVERT_OUTPUT)
            })?
            .trim()
            .to_string();
        // return selected task
        if selected_task.is_empty() {
            Ok(None)
        } else {
            Ok(Some(selected_task))
        }
    }
}
