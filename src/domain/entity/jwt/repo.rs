use crate::result::Result;
use super::entity::Jwt;

#[async_trait::async_trait]
pub trait JwtRepo {
    async fn check_valid(&self, token: &str) -> Result<bool>;
    async fn save(&self, jwt: &Jwt) -> Result<String>;
}
