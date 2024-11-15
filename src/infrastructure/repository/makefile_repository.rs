use crate::domain::model::task::Task;
use std::fs::File;
use std::io::Read;
use toml::Value;

pub struct MakefileRepository;

impl MakefileRepository {
    pub fn new() -> Self {
        MakefileRepository
    }

    pub fn get_tasks(&self) -> Result<Vec<Task>, std::io::Error> {
        let mut file = File::open("Makefile.toml")?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        let value: Value = toml::from_str(&contents).map_err(|err| {
            std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Failed to parse TOML: {}", err),
            )
        })?;
        let tasks = value["tasks"].as_table().ok_or_else(|| {
            std::io::Error::new(std::io::ErrorKind::Other, "Failed to read tasks from TOML")
        })?;

        Ok(tasks
            .keys()
            .map(|s| Task {
                name: s.to_string(),
            })
            .collect())
    }
}
