use crate::domain::value_object::task_name::TaskName;

#[derive(Clone, Debug, PartialEq)]
pub struct Task {
    name: TaskName,
    description: Option<String>,
}

impl Task {
    pub fn new(name: String, description: Option<String>) -> Result<Self, &'static str> {
        Ok(Self {
            name: TaskName::new(name)?,
            description,
        })
    }

    pub fn name(&self) -> &str {
        self.name.value()
    }
}
