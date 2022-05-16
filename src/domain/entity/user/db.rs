use super::entity::User;
use super::repo::UserRepo;
use crate::result::Result;
use async_trait::async_trait;
use chrono::Local;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

#[derive(Clone)]
pub struct UserDb(pub Pool<Postgres>);

impl From<Pool<Postgres>> for UserDb {
    fn from(pool: Pool<Postgres>) -> Self {
        UserDb(pool)
    }
}

#[async_trait]
impl UserRepo for UserDb {
    async fn find_by_id(&self, id: &str) -> Result<Option<User>> {
        let row: Option<(Uuid, String, Option<String>, Option<String>)> =
            sqlx::query_as(r#"SELECT id, name, email, phone FROM users WHERE id = $1"#)
                .bind(id)
                .fetch_optional(&self.0)
                .await?;
        Ok(row.map(|row| User {
            id: row.0,
            name: row.1,
            email: row.2,
            phone: row.3,
        }))
    }

    async fn find_by_name(&self, name: &str) -> Result<Option<User>> {
        let row: Option<(Uuid, String, Option<String>, Option<String>)> =
            sqlx::query_as(r#"SELECT id, name, email, phone FROM users WHERE name = $1"#)
                .bind(name)
                .fetch_optional(&self.0)
                .await?;
        Ok(row.map(|(id, name, email, phone)| User {
            id,
            name,
            email,
            phone,
        }))
    }

    async fn save(&self, user: &User, salt: &str, pw_digest: &str) -> Result<Uuid> {
        let now = Local::now();

        let mut transaction = self.0.begin().await?;

        // 先插入用户信息
        let row: (Uuid,) =
            sqlx::query_as(r#"INSERT INTO users (name, email, phone, created_at, updated_at) VALUES($1, $2, $3, $4, $4) RETURNING id"#)
                .bind(&user.name)
                .bind(&user.email)
                .bind(&user.phone)
                .bind(&now)
                .fetch_one(&mut transaction)
                .await?;

        // 再插入认证信息
        sqlx::query(r#"INSERT INTO authentications (user_id, grant_type, token, salt, created_at, updated_at) VALUES($1, $2, $3, $4, $5, $5)"#)
            .bind(&row.0)
            .bind("password")
            .bind(&pw_digest)
            .bind(&salt)
            .bind(&now)
            .execute(&mut transaction)
            .await?;

        transaction.commit().await?;

        Ok(row.0)
    }
}
