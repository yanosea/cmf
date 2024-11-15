use crate::infrastructure::repository::fzf_repository::FzfRepository;

pub fn select_task_with_fzf(
    tasks: &[String],
    args: &[String],
) -> Result<Option<String>, std::io::Error> {
    FzfRepository::new().select_task(tasks, args)
}
