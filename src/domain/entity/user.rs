use crate::error::Error;
use crate::result::Result;
use async_trait::async_trait;
use chrono::Duration;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct User {
    pub id: Option<Uuid>,
    pub name: String,
    pub email: Option<String>,
    pub phone: Option<String>,
}

impl User {
    pub async fn create(
        &mut self,
        repo: &impl UserRepository,
        salt: &str,
        pw_digest: &str,
    ) -> Result<()> {
        let user = repo.find_by_name(&self.name).await?;
        if user.is_some() {
            return Err(Error::UserNameExistsError);
        }

        let user_id = repo.save(self, salt, pw_digest).await?;
        self.id = Some(user_id);
        Ok(())
    }
}

#[async_trait]
pub trait UserRepository {
    async fn find_by_id(&self, id: &str) -> Result<Option<User>>;
    async fn find_by_name(&self, id: &str) -> Result<Option<User>>;
    async fn save(&self, user: &User, salt: &str, pw_digest: &str) -> Result<Uuid>;
}
