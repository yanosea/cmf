use crate::domain::model::task::Task;
use crate::infrastructure::repository::makefile_repository::MakefileRepository;

pub fn get_tasks_from_makefile() -> Result<Vec<Task>, std::io::Error> {
    MakefileRepository::new().get_tasks()
}
