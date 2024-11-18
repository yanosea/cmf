use async_trait::async_trait;

#[async_trait]
pub trait FzfSelector {
    async fn select_from_list(
        &self,
        items: &[String],
        args: &[String],
    ) -> Result<Option<String>, Box<dyn std::error::Error>>;
}
