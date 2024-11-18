use crate::application::usecase::{ExecuteTaskUseCase, GetTasksUseCase, SelectTaskUseCase};
use crate::infrastructure::repository::{FzfRepository, MakefileRepository};
use crate::interface::presenter::CliPresenter;

use std::env;

pub struct CliController {
    get_tasks_usecase: GetTasksUseCase<MakefileRepository>,
    select_task_usecase: SelectTaskUseCase<FzfRepository>,
    execute_task_usecase: ExecuteTaskUseCase,
    presenter: CliPresenter,
}

impl CliController {
    pub fn new() -> Self {
        Self {
            get_tasks_usecase: GetTasksUseCase::new(MakefileRepository::new(
                "Makefile.toml".into(),
            )),
            select_task_usecase: SelectTaskUseCase::new(FzfRepository::new()),
            execute_task_usecase: ExecuteTaskUseCase::new(),
            presenter: CliPresenter::new(),
        }
    }

    pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        let tasks = self.get_tasks_usecase.execute().await?;
        let task_names: Vec<String> = tasks.iter().map(|t| t.name().to_string()).collect();

        let args: Vec<String> = env::args().skip(1).collect();

        let selected_task = self.select_task_usecase.execute(&task_names, &args).await?;

        match selected_task {
            Some(task) => {
                if self.presenter.confirm_task_execution(&task)? {
                    self.execute_task_usecase.execute(&task).await?;
                    self.presenter.show_success(&task);
                } else {
                    self.presenter.show_cancelled();
                }
            }
            None => self.presenter.show_no_selection(),
        }

        Ok(())
    }
}
