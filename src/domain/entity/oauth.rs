use crate::db::user::UserRepositoryDb;
use crate::domain::entity::{Jwt, JwtRepository, User, UserRepository};
use crate::error::Error;
use crate::result::Result;
use crate::utils::{random_str, sha1};
use chrono::Duration;
use uuid::Uuid;

pub enum Oauth {
    Password(String, String), // salt, pw_digest
    Wechat,
    Github,
}

impl Oauth {
    pub async fn register_by_password(
        username: &str,
        password: &str,
        user_repo: &impl UserRepository,
        jwt_repo: &impl JwtRepository,
    ) -> Result<Jwt> {
        let mut user = User {
            name: username.to_string(),
            ..User::default()
        };

        // 创建用户及认证信息
        let salt = random_str(7);
        let salted_pw = password.to_string() + &salt;
        let pw_digest = sha1(&salted_pw);
        user.create(user_repo, &salt, &pw_digest).await?;

        let jwt = Jwt::new(&user.id.unwrap(), &user.name, Duration::days(1));
        jwt.save(jwt_repo).await?;
        Ok(jwt)
    }

    pub async fn login_with_password(
        user_repo: &impl UserRepository,
        oauth_repo: &impl OauthRepository,
        jwt_repo: &impl JwtRepository,
        username: &str,
        password: &str,
    ) -> Result<Jwt> {
        let user = user_repo
            .find_by_name(username)
            .await?
            .ok_or(Error::AuthenticationFailedError)?;

        let user_id = user.id.as_ref().unwrap();
        let oauth = oauth_repo
            .find(user_id, "password")
            .await?
            .ok_or(Error::AuthenticationFailedError)?;
        match oauth {
            Oauth::Password(salt, pw_digest) => {
                let salted_pw = password.to_string() + &salt;
                let expected_pw_digest = sha1(&salted_pw);
                if pw_digest.eq(&expected_pw_digest) {
                    // 生成 jwt 并持久化
                    let jwt = Jwt::new(user_id, &user.name, Duration::days(1));
                    jwt.save(jwt_repo).await?;

                    Ok(jwt)
                } else {
                    Err(Error::AuthenticationFailedError)
                }
            }
            _ => Err(Error::AuthenticationFailedError),
        }
    }
}

#[async_trait::async_trait]
pub trait OauthRepository {
    async fn find(&self, user_id: &Uuid, grant_type: &str) -> Result<Option<Oauth>>;
}
