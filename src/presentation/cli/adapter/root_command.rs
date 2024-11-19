use super::cmf;
use crate::application::usecase::{ExecuteTaskUseCase, GetTasksUseCase, SelectTaskUseCase};
use crate::infrastructure::gateway::{fzf::FzfGateway, makefile::MakefileGateway};
use crate::presentation::cli::adapter::formatter::CliFormatter;
use crate::presentation::cli::adapter::presenter::CliPresenter;
use std::env;

pub struct RootCommand {
    makefile_usecase: GetTasksUseCase<MakefileGateway>,
    cmf_usecase: SelectTaskUseCase<FzfGateway>,
    cargo_usecase: ExecuteTaskUseCase,
    cli_presenter: CliPresenter,
    cli_formatter: CliFormatter,
}

impl RootCommand {
    pub fn new() -> Self {
        Self {
            makefile_usecase: cmf::new_makefile_usecase(),
            cmf_usecase: cmf::new_cmf_usecase(),
            cargo_usecase: cmf::new_cargo_usecase(),
            cli_presenter: cmf::new_cli_presenter(),
            cli_formatter: cmf::new_cli_formatter(),
        }
    }

    pub async fn execute(&self) -> Result<(), Box<dyn std::error::Error>> {
        let tasks = self.makefile_usecase.execute().await?;
        let task_names: Vec<String> = tasks.iter().map(|t| t.name().to_string()).collect();
        let args: Vec<String> = env::args().skip(1).collect();

        let selected_task = self.cmf_usecase.execute(&task_names, &args).await?;

        match selected_task {
            Some(task) => {
                let formatted_task = self.cli_formatter.format_task_name(&task);
                if self.cli_presenter.confirm_task_execution(&formatted_task)? {
                    self.cargo_usecase.execute(&task).await?;
                    self.cli_presenter.show_success(&formatted_task);
                } else {
                    self.cli_presenter.show_cancelled();
                }
            }
            None => self.cli_presenter.show_no_selection(),
        }

        Ok(())
    }
}
