use crate::domain::repository::FzfRepository as FzfRepositoryTrait;
use async_trait::async_trait;
use tokio::process::Command;

pub struct FzfRepositoryImpl;

impl FzfRepositoryImpl {
    pub fn new() -> Self {
        Self
    }

    fn process_args(&self, args: &[String]) -> Vec<String> {
        if args.is_empty() {
            return Vec::new();
        }
        vec!["--query".to_string(), args.join(" ")]
    }
}

#[async_trait]
impl FzfRepositoryTrait for FzfRepositoryImpl {
    async fn select_from_list(
        &self,
        items: &[String],
        args: &[String],
    ) -> Result<Option<String>, Box<dyn std::error::Error>> {
        let input = items.join("\n");
        let mut command = Command::new("fzf");

        let processed_args = self.process_args(args);
        for arg in processed_args {
            command.arg(arg);
        }

        let mut child = command
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .spawn()?;

        if let Some(mut stdin) = child.stdin.take() {
            use tokio::io::AsyncWriteExt;
            stdin.write_all(input.as_bytes()).await?;
        }

        let output = child.wait_with_output().await?;

        if output.status.success() {
            let selected = String::from_utf8(output.stdout)?;
            let trimmed = selected.trim();
            if trimmed.is_empty() {
                Ok(None)
            } else {
                Ok(Some(trimmed.to_string()))
            }
        } else {
            Ok(None)
        }
    }
}
