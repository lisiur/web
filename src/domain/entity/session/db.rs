use super::entity::Session;
use super::repo::SessionRepo;
use crate::result::Result;
use async_trait::async_trait;
use chrono::{Local, TimeZone};
use sqlx::{Pool, Postgres};
use uuid::Uuid;

pub struct SessionDb(pub Pool<Postgres>);

impl From<Pool<Postgres>> for SessionDb {
    fn from(pool: Pool<Postgres>) -> Self {
        SessionDb(pool)
    }
}

#[async_trait]
impl SessionRepo for SessionDb {
    async fn check_valid(&self, token: &str) -> Result<bool> {
        let row: (Option<bool>,) = sqlx::query_as(r#"SELECT invalid from sessions WHERE jwt = $1"#)
            .bind(token)
            .fetch_one(&self.0)
            .await?;
        match row.0 {
            Some(true) => Ok(true),
            _ => Ok(false),
        }
    }

    async fn save(&self, session: &Session) -> Result<String> {
        let now = Local::now();
        let expired_at = Local.timestamp(session.expired_at, 0);
        let row: (Uuid,) =
            sqlx::query_as(r#"INSERT INTO sessions (user_id, jwt, expired_at, created_at, updated_at) VALUES($1, $2, $3, $4, $4) RETURNING id"#)
                .bind(&session.user_id)
                .bind(&session.token)
                .bind(&expired_at)
                .bind(&now)
                .fetch_one(&self.0)
                .await?;
        Ok(row.0.to_string())
    }

    async fn remove(&self, token: &str) -> Result<()> {
        sqlx::query(r#"DELETE FROM sessions WHERE token = $1"#)
            .bind(token)
            .execute(&self.0)
            .await?;
        Ok(())
    }
}
