use crate::domain::model::task::Task;
use crate::infrastructure::repository::constant::*;
use std::fs::File;
use std::io::Read;
use toml::Value;

// MakefileRepository is a struct to get tasks from Makefile.toml.
pub struct MakefileRepository;

// Implementation of MakefileRepository.
impl MakefileRepository {
    // new is a constructor of MakefileRepository.
    pub fn new() -> Self {
        MakefileRepository
    }

    // get_tasks is a method to get tasks from Makefile.
    pub fn get_tasks(&self) -> Result<Vec<Task>, std::io::Error> {
        // read Makefile.toml
        let mut file = File::open(PATH_MAKEFILE)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        // parse Makefile.toml
        let value: Value = toml::from_str(&contents).map_err(|err| {
            std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("{}: {}", ERROR_FAILED_TO_PARSE_TOML, err),
            )
        })?;

        // get tasks from Makefile.toml
        let tasks = value[KEY_TASKS].as_table().ok_or_else(|| {
            std::io::Error::new(std::io::ErrorKind::Other, ERROR_FAILED_TO_READ_TASKS)
        })?;

        // return tasks
        Ok(tasks
            .keys()
            .map(|s| Task {
                name: s.to_string(),
            })
            .collect())
    }
}
