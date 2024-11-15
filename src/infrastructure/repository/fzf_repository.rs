use crate::infrastructure::repository::constant::*;
use std::io::Write;
use std::process::{Command, Stdio};

pub struct FzfRepository;

impl FzfRepository {
    pub fn new() -> Self {
        FzfRepository
    }

    pub fn select_task(
        &self,
        tasks: &[String],
        args: &[String],
    ) -> Result<Option<String>, std::io::Error> {
        let tasks = tasks.join("\n");
        let concatenated_args = args.join(" ");

        let mut child = Command::new(COMMAND_FZF)
            .arg(ARG_QUERY)
            .arg(&concatenated_args)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .map_err(|_| {
                std::io::Error::new(std::io::ErrorKind::Other, ERROR_FAILED_TO_SPAWN_FZF)
            })?;

        child
            .stdin
            .as_mut()
            .ok_or_else(|| {
                std::io::Error::new(std::io::ErrorKind::Other, ERROR_FAILED_TO_OPEN_STDIN)
            })?
            .write_all(tasks.as_bytes())?;

        let output = child.wait_with_output()?;

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
