use crate::domain::entity::Oauth;
use crate::domain::repo::OauthRepo;
use crate::error::Error;
use crate::prelude::DbPool;
use crate::result::Result;
use uuid::Uuid;

pub struct OauthDb<'a>(pub &'a DbPool);

#[async_trait::async_trait]
impl OauthRepo for OauthDb<'_> {
    async fn find(&self, user_id: &Uuid, grant_type: &str) -> Result<Option<Oauth>> {
        let row: Option<(String, String)> = sqlx::query_as(
            r#"SELECT salt, token FROM authentications WHERE user_id = $1 AND grant_type = $2"#,
        )
        .bind(user_id)
        .bind(grant_type)
        .fetch_optional(self.0)
        .await?;

        Ok(row.map(|row| Oauth::Password(row.0, row.1)))
    }
}
