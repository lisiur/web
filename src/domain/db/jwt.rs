use crate::domain::entity::Jwt;
use crate::domain::repo::JwtRepo;
use crate::error::Error;
use crate::result::Result;
use async_trait::async_trait;
use chrono::{Local, TimeZone};
use sqlx::{Pool, Postgres};
use uuid::Uuid;

pub struct JwtDb<'a>(pub &'a Pool<Postgres>);

#[async_trait]
impl<'a> JwtRepo for JwtDb<'a> {
    async fn check_valid(&self, token: &str) -> Result<bool> {
        let row: (Option<bool>,) = sqlx::query_as(r#"SELECT invalid from sessions WHERE jwt = $1"#)
            .bind(token)
            .fetch_one(self.0)
            .await?;
        match row.0 {
            Some(true) => Ok(true),
            _ => Ok(false),
        }
    }

    async fn save(&self, jwt: &Jwt) -> Result<String> {
        let now = Local::now();
        let expired_at = Local.timestamp(jwt.expired_at, 0);
        let row: (Uuid,) =
            sqlx::query_as(r#"INSERT INTO sessions (user_id, jwt, expired_at, created_at, updated_at) VALUES($1, $2, $3, $4, $4) RETURNING id"#)
                .bind(&jwt.user_id)
                .bind(&jwt.token)
                .bind(&expired_at)
                .bind(&now)
                .fetch_one(self.0)
                .await?;
        Ok(row.0.to_string())
    }
}
