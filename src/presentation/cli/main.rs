use cmf::presentation::cli::adapter::root_command::RootCommand;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let command = RootCommand::new();
    command.execute().await
}
