use crate::infrastructure::repository::fzf_repository::FzfRepository;

// select_task_with_fzf is a function to select a task with fzf.
// tasks &Vec<String> : is a reference to a vector of tasks.
// args  &Vec<String> : is a reference to a vector of arguments.
pub fn select_task_with_fzf(
    tasks: &[String],
    args: &[String],
) -> Result<Option<String>, std::io::Error> {
    // select a task with fzf
    FzfRepository::new().select_task(tasks, args)
}
