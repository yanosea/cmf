use colored::Colorize;
use std::io::{self, BufRead, Write};
use std::process::{Command, Stdio};

pub fn select_task_with_fzf(
    tasks: &Vec<String>,
    args: &Vec<String>,
) -> Result<Option<Option<String>>, std::io::Error> {
    let tasks = tasks.join("\n");
    let concatenated_args = args.join(" ");

    let mut child = Command::new("fzf")
        .arg("--query")
        .arg(&concatenated_args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "Failed to spawn fzf"))?;

    child
        .stdin
        .as_mut()
        .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::Other, "Failed to open stdin"))?
        .write_all(tasks.as_bytes())?;

    let output = child.wait_with_output()?;

    let selected_task = String::from_utf8(output.stdout)
        .map_err(|_| {
            std::io::Error::new(
                std::io::ErrorKind::Other,
                "Failed to convert output to UTF-8",
            )
        })?
        .trim()
        .to_string();
    if selected_task.is_empty() {
        Ok(None)
    } else {
        match confirm_selection(selected_task)? {
            Some(task) => Ok(Some(Some(task))),
            None => Ok(Some(None)),
        }
    }
}

fn confirm_selection(task: String) -> Result<Option<String>, std::io::Error> {
    print!("You selected: [{}]. Are you OK? (y/N) : ", task.red());
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().lock().read_line(&mut input)?;
    match input.trim().to_lowercase().as_str() {
        "y" => Ok(Some(task)),
        _ => Ok(None),
    }
}
