use crate::domain::model::task::Task;
use crate::infrastructure::repository::makefile_repository::MakefileRepository;

// get_tasks_from_makefile is a function to get tasks from Makefile.
pub fn get_tasks_from_makefile() -> Result<Vec<Task>, std::io::Error> {
    // get tasks from Makefile.toml
    MakefileRepository::new().get_tasks()
}
