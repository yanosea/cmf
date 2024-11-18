pub struct ExecuteTaskUseCase {
    cargo_command: String,
}

impl ExecuteTaskUseCase {
    pub fn new() -> Self {
        Self {
            cargo_command: String::from("cargo"),
        }
    }

    pub async fn execute(&self, task_name: &str) -> Result<(), Box<dyn std::error::Error>> {
        let status = tokio::process::Command::new(&self.cargo_command)
            .arg("make")
            .arg(task_name)
            .status()
            .await?;

        if !status.success() {
            return Err(format!("Failed to execute task: {}", task_name).into());
        }

        Ok(())
    }
}
