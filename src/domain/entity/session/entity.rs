use crate::result::Result;
use async_trait::async_trait;
use chrono::{DateTime, Duration, Local, TimeZone, Utc};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub struct Session {
    pub user_id: Uuid,
    pub user_name: String,
    pub expired_at: i64,
    pub token: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Claims {
    pub user_id: Uuid,
    pub user_name: String,
    pub exp: usize,
}

impl Session {
    /// 创建 session
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
        Self {
            user_id: user_id.clone(),
            user_name: user_name.to_string(),
            expired_at: exp,
            token,
        }
    }

    /// 解析 session
    pub fn try_decode(token: &str) -> Result<Claims> {
        decode::<Claims>(
            token,
            &DecodingKey::from_secret("secret".as_ref()),
            &Validation::default(),
        )
        .map(|token| token.claims)
        .map_err(Into::into)
    }
}
