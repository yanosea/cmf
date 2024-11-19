use async_trait::async_trait;

#[async_trait]
pub trait FzfRepository {
    async fn select_from_list(
        &self,
        items: &[String],
        args: &[String],
    ) -> Result<Option<String>, Box<dyn std::error::Error>>;
}
