use crate::domain::model::Task;
use crate::presentation::cli::adapter::dto::TaskDto;

pub struct TaskMapper;

impl TaskMapper {
    pub fn to_dto(task: &Task) -> TaskDto {
        TaskDto {
            name: task.name().to_string(),
            description: None,
        }
    }

    pub fn to_dtos(tasks: &[Task]) -> Vec<TaskDto> {
        tasks.iter().map(|t| Self::to_dto(t)).collect()
    }
}
