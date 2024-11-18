use crate::application::port::repository::TaskRepository;
use crate::domain::entity::Task;

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
