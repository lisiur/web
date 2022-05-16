use crate::result::Result;
use super::entity::Session;

#[async_trait::async_trait]
pub trait SessionRepo {
    async fn check_valid(&self, token: &str) -> Result<bool>;
    async fn save(&self, session: &Session) -> Result<String>;
}
