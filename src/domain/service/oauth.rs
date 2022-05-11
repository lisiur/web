use crate::domain::entity::{Jwt, User, Oauth};
use crate::domain::repo::{JwtRepo, UserRepo, OauthRepo};
use crate::domain::db::{JwtDb, UserDb};
use crate::prelude::DbPool;
use crate::result::Result;
use crate::error::Error;
use crate::utils::{random_str, sha1};
use chrono::Duration;
use uuid::Uuid;

pub struct RegisterByPasswordService<'a> {
    user_repo: &'a dyn UserRepo,
    jwt_repo: &'a dyn JwtRepo,
}

impl <'a> RegisterByPasswordService<'a> {
    pub fn new(user_repo: &'a dyn UserRepo, jwt_repo: &'a dyn JwtRepo) -> Self {
        Self {
            user_repo,
            jwt_repo,
        }
    }

    pub async fn exec(&self, username: &str, password: &str, email: &str) -> Result<Jwt> {
        let mut user = User {
            name: username.to_string(),
            email: Some(email.to_string()),
            ..User::default()
        };

        // 创建用户及认证信息
        let salt = random_str(7);
        let salted_pw = password.to_string() + &salt;
        let pw_digest = sha1(&salted_pw);

        let exist_user = self.user_repo.find_by_name(username).await?;
        if exist_user.is_some() {
            return Err(Error::UserNameExistsError);
        }

        let user_id = self.user_repo.save(&user, &salt, &pw_digest).await?;
        user.id = user_id;

        let jwt = Jwt::new(&user.id, &user.name, Duration::days(1));
        self.jwt_repo.save(&jwt).await?;

        Ok(jwt)
    }
}

pub struct LoginByPasswordService<'a> {
    user_repo: &'a dyn UserRepo,
    jwt_repo: &'a dyn JwtRepo,
    oauth_repo: &'a dyn OauthRepo,
}

impl <'a> LoginByPasswordService<'a> {
    pub fn new(user_repo: &'a dyn UserRepo, jwt_repo: &'a dyn JwtRepo, oauth_repo: &'a dyn OauthRepo) -> Self {
        Self {
            user_repo,
            jwt_repo,
            oauth_repo,
        }
    }

    pub async fn exec(&self, username: &str, password: &str) -> Result<Jwt> {
        let user = self.user_repo
            .find_by_name(username)
            .await?
            .ok_or(Error::AuthenticationFailedError)?;

        let oauth = self.oauth_repo
            .find(&user.id, "password")
            .await?
            .ok_or(Error::AuthenticationFailedError)?;
        match oauth {
            Oauth::Password(salt, pw_digest) => {
                let salted_pw = password.to_string() + &salt;
                let expected_pw_digest = sha1(&salted_pw);
                if pw_digest.eq(&expected_pw_digest) {
                    // 生成 jwt 并持久化
                    let jwt = Jwt::new(&user.id, &user.name, Duration::days(1));
                    self.jwt_repo.save(&jwt).await?;

                    Ok(jwt)
                } else {
                    Err(Error::AuthenticationFailedError)
                }
            }
            _ => Err(Error::AuthenticationFailedError),
        }
    }
}
