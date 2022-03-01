use crate::result::Result;
use async_trait::async_trait;
use chrono::{DateTime, Duration, Local, TimeZone, Utc};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub struct Jwt {
    pub user_id: Uuid,
    pub user_name: String,
    pub expired_at: i64,
    pub token: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Claims {
    user_id: Uuid,
    user_name: String,
    exp: usize,
}

impl Jwt {
    /// 创建 jwt
    pub fn new(user_id: &Uuid, user_name: &str, expired_in: Duration) -> Self {
        let exp = Utc::now()
            .checked_add_signed(expired_in)
            .expect("invalid timestamp")
            .timestamp();
        let claims = Claims {
            user_id: user_id.clone(),
            user_name: user_name.to_string(),
            exp: exp as usize,
        };
        let header = Header::new(Algorithm::HS512);
        let token = encode(
            &header,
            &claims,
            &EncodingKey::from_secret("secret".as_ref()),
        )
        .unwrap();
        Jwt {
            user_id: user_id.clone(),
            user_name: user_name.to_string(),
            expired_at: exp,
            token,
        }
    }

    /// 解码 jwt
    pub fn try_decode(token: &str) -> Result<Claims> {
        decode::<Claims>(
            token,
            &DecodingKey::from_secret("secret".as_ref()),
            &Validation::default(),
        )
        .map(|token| token.claims)
        .map_err(Into::into)
    }

    /// 持久化 jwt
    pub async fn save(&self, repo: &impl JwtRepository) -> Result<String> {
        repo.save(self).await
    }
}

#[async_trait]
pub trait JwtRepository {
    async fn check_valid(&self, token: &str) -> Result<bool>;
    async fn save(&self, jwt: &Jwt) -> Result<String>;
}
