use crate::domain::model::Task;
use crate::domain::repository::TaskRepository;
use async_trait::async_trait;
use tokio::fs;

pub struct MakefileGateway {
    file_path: String,
}

impl MakefileGateway {
    pub fn new(file_path: String) -> Self {
        Self { file_path }
    }
}

#[async_trait]
impl TaskRepository for MakefileGateway {
    async fn get_tasks(&self) -> Result<Vec<Task>, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(&self.file_path).await?;
        let value: toml::Value = toml::from_str(&content)?;

        let tasks = value["tasks"]
            .as_table()
            .ok_or("No tasks found in Makefile.toml")?;

        let mut result = Vec::new();
        for name in tasks.keys() {
            if let Ok(task) = Task::new(name.to_string(), None) {
                result.push(task);
            }
        }

        Ok(result)
    }
}
