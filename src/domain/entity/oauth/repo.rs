use super::entity::Oauth;
use crate::result::Result;
use uuid::Uuid;

#[async_trait::async_trait]
pub trait OauthRepo {
    async fn find(&self, user_id: &Uuid, grant_type: &str) -> Result<Option<Oauth>>;
}
