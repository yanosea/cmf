use crate::domain::repository::FzfRepository;

pub struct SelectTaskUseCase<F: FzfRepository> {
    selector: F,
}

impl<F: FzfRepository> SelectTaskUseCase<F> {
    pub fn new(selector: F) -> Self {
        Self { selector }
    }

    pub async fn execute(
        &self,
        tasks: &[String],
        args: &[String],
    ) -> Result<Option<String>, Box<dyn std::error::Error>> {
        self.selector.select_from_list(tasks, args).await
    }
}
