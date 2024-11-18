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

    // get_tasks is a function to get tasks from Makefile.toml.
    pub fn get_tasks(&self) -> Result<Vec<Task>, std::io::Error> {
        let contents = self.read_makefile_contents()?;
        let value = self.parse_toml(&contents)?;
        let tasks = self.extract_tasks(&value)?;
        Ok(tasks)
    }

    // read_makefile_contents is a function to read the contents of Makefile.toml.
    fn read_makefile_contents(&self) -> Result<String, std::io::Error> {
        // open Makefile.toml (current directory)
        let mut file = File::open(PATH_MAKEFILE)?;
        let mut contents = String::new();
        // read the contents of Makefile.toml
        file.read_to_string(&mut contents)?;
        // return the contents of Makefile.toml
        Ok(contents)
    }

    // parse_toml is a function to parse the contents of Makefile.toml.
    fn parse_toml(&self, contents: &str) -> Result<Value, std::io::Error> {
        // parse the contents of Makefile.toml
        toml::from_str(contents).map_err(|err| {
            std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("{}: {}", ERROR_FAILED_TO_PARSE_TOML, err),
            )
        })
    }

    // extract_tasks is a function to extract tasks from the parsed contents of Makefile.toml.
    fn extract_tasks(&self, value: &Value) -> Result<Vec<Task>, std::io::Error> {
        // extract tasks from the parsed contents of Makefile.toml
        let tasks = value[KEY_TASKS].as_table().ok_or_else(|| {
            std::io::Error::new(std::io::ErrorKind::Other, ERROR_FAILED_TO_READ_TASKS)
        })?;
        // return the names of the tasks
        Ok(tasks
            .keys()
            .map(|s| Task {
                name: s.to_string(),
            })
            .collect())
    }
}
