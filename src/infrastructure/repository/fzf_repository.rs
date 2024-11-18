use crate::application::port::repository::FzfSelector;
use async_trait::async_trait;
use tokio::process::Command;

pub struct FzfRepository;

impl FzfRepository {
    pub fn new() -> Self {
        Self
    }

    fn process_args(&self, args: &[String]) -> Vec<String> {
        let mut processed_args = Vec::new();
        let mut i = 0;
        while i < args.len() {
            if args[i].starts_with("--") {
                processed_args.push(args[i].clone());
                if i + 1 < args.len() {
                    processed_args.push(args[i + 1].clone());
                }
                i += 2;
            } else {
                processed_args.push("--query".to_string());
                processed_args.push(args[i].clone());
                i += 1;
            }
        }
        processed_args
    }
}

#[async_trait]
impl FzfSelector for FzfRepository {
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
