use crate::domain::model::Task;
use crate::domain::repository::TaskRepository;

pub struct GetTasksUseCase<R: TaskRepository> {
    repository: R,
}

impl<R: TaskRepository> GetTasksUseCase<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn execute(&self) -> Result<Vec<Task>, Box<dyn std::error::Error>> {
        self.repository.get_tasks().await
    }
}
