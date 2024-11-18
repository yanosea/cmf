use crate::domain::entity::Task;
use async_trait::async_trait;

#[async_trait]
pub trait TaskRepository {
    async fn get_tasks(&self) -> Result<Vec<Task>, Box<dyn std::error::Error>>;
}
