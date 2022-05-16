use super::entity::User;
use crate::result::Result;
use uuid::Uuid;

#[async_trait::async_trait]
pub trait UserRepo {
    async fn find_by_id(&self, id: &str) -> Result<Option<User>>;
    async fn find_by_name(&self, id: &str) -> Result<Option<User>>;
    async fn save(&self, user: &User, salt: &str, pw_digest: &str) -> Result<Uuid>;
}
