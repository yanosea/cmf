use crate::application::usecase::{ExecuteTaskUseCase, GetTasksUseCase, SelectTaskUseCase};
use crate::infrastructure::gateway::{fzf::FzfGateway, makefile::MakefileGateway};
use crate::presentation::cli::adapter::formatter::CliFormatter;
use crate::presentation::cli::adapter::presenter::CliPresenter;
use std::path::PathBuf;

pub fn new_makefile_usecase() -> GetTasksUseCase<MakefileGateway> {
    let makefile_path = PathBuf::from("Makefile.toml");
    let repository = MakefileGateway::new(makefile_path.to_str().unwrap().to_string());
    GetTasksUseCase::new(repository)
}

pub fn new_cmf_usecase() -> SelectTaskUseCase<FzfGateway> {
    let gateway = FzfGateway::new();
    SelectTaskUseCase::new(gateway)
}

pub fn new_cargo_usecase() -> ExecuteTaskUseCase {
    ExecuteTaskUseCase::new()
}

pub fn new_cli_presenter() -> CliPresenter {
    CliPresenter::new()
}

pub fn new_cli_formatter() -> CliFormatter {
    CliFormatter::new()
}
