use crate::application::usecase::{ExecuteTaskUseCase, GetTasksUseCase, SelectTaskUseCase};
use crate::infrastructure::repository::{fzf::FzfRepositoryImpl, makefile::MakefileRepository};
use crate::presentation::cli::adapter::formatter::CmfFormatter;
use crate::presentation::cli::adapter::presenter::CmfPresenter;
use std::path::PathBuf;

pub fn new_makefile_usecase() -> GetTasksUseCase<MakefileRepository> {
    let makefile_path = PathBuf::from("Makefile.toml");
    let repository = MakefileRepository::new(makefile_path.to_str().unwrap().to_string());
    GetTasksUseCase::new(repository)
}

pub fn new_cmf_usecase() -> SelectTaskUseCase<FzfRepositoryImpl> {
    let repository = FzfRepositoryImpl::new();
    SelectTaskUseCase::new(repository)
}

pub fn new_cargo_usecase() -> ExecuteTaskUseCase {
    ExecuteTaskUseCase::new()
}

pub fn new_cli_presenter() -> CmfPresenter {
    CmfPresenter::new()
}

pub fn new_cli_formatter() -> CmfFormatter {
    CmfFormatter::new()
}
