use crate::application::port::repository::FzfSelector;

pub struct SelectTaskUseCase<F: FzfSelector> {
    selector: F,
}

impl<F: FzfSelector> SelectTaskUseCase<F> {
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
