use super::cmf_adapter;
use crate::application::usecase::{ExecuteTaskUseCase, GetTasksUseCase, SelectTaskUseCase};
use crate::infrastructure::repository::{fzf::FzfRepositoryImpl, makefile::MakefileRepository};
use crate::presentation::cli::adapter::formatter::CmfFormatter;
use crate::presentation::cli::adapter::presenter::CmfPresenter;
use std::env;

pub struct RootCommand {
    makefile_usecase: GetTasksUseCase<MakefileRepository>,
    cmf_usecase: SelectTaskUseCase<FzfRepositoryImpl>,
    cargo_usecase: ExecuteTaskUseCase,
    cli_presenter: CmfPresenter,
    cli_formatter: CmfFormatter,
}

impl RootCommand {
    pub fn new() -> Self {
        Self {
            makefile_usecase: cmf_adapter::new_makefile_usecase(),
            cmf_usecase: cmf_adapter::new_cmf_usecase(),
            cargo_usecase: cmf_adapter::new_cargo_usecase(),
            cli_presenter: cmf_adapter::new_cli_presenter(),
            cli_formatter: cmf_adapter::new_cli_formatter(),
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
