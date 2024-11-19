#[derive(Clone, Debug, PartialEq)]
pub struct TaskName(String);

impl TaskName {
    pub fn new(value: String) -> Result<Self, &'static str> {
        if value.trim().is_empty() {
            return Err("Task name cannot be empty");
        }
        Ok(Self(value))
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}
