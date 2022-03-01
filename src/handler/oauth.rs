use crate::db::authentication::AuthenticationRepoDb;
use crate::db::jwt::JwtRepositoryDb;
use crate::db::user::UserRepositoryDb;
use crate::domain::entity::{Jwt, Oauth, OauthRepository, User, UserRepository};
use crate::prelude::*;
use crate::response::JsonResponseResult;

/// 注册参数
#[derive(Deserialize, Debug)]
pub struct RegisterParams {
    /// 用户名
    username: String,
    /// 密码
    password: String,
}

#[derive(Serialize, Debug)]
pub struct UserToken {
    pub user_id: Uuid,
    pub user_name: String,
    pub expired_at: i64,
    pub token: String,
}

/// 注册
#[post("/register")]
pub async fn register(
    pool: Data<DbPool>,
    params: Json<RegisterParams>,
) -> JsonResponseResult<UserToken> {
    let user_repo = UserRepositoryDb(&pool);
    let jwt_repo = JwtRepositoryDb(&pool);
    let jwt =
        Oauth::register_by_password(&params.username, &params.password, &user_repo, &jwt_repo)
            .await?;
    JsonResponse::ok(UserToken {
        user_id: jwt.user_id,
        user_name: jwt.user_name,
        expired_at: jwt.expired_at,
        token: jwt.token,
    })
}

/// 登录参数
#[derive(Deserialize, Debug)]
pub struct LoginParams {
    /// 用户名
    username: String,
    /// 密码
    password: String,
}

/// 登录
#[post("/login")]
pub async fn login(pool: Data<DbPool>, params: Json<LoginParams>) -> JsonResponseResult<UserToken> {
    let user_repo = UserRepositoryDb(&pool);
    let oauth_repo = AuthenticationRepoDb(&pool);
    let jwt_repo = JwtRepositoryDb(&pool);
    let jwt = Oauth::login_with_password(
        &user_repo,
        &oauth_repo,
        &jwt_repo,
        &params.username,
        &params.password,
    )
    .await?;

    JsonResponse::ok(UserToken {
        user_id: jwt.user_id,
        user_name: jwt.user_name,
        expired_at: jwt.expired_at,
        token: jwt.token,
    })
}

/// 登出
pub async fn logout(pool: Data<DbPool>) -> Result<()> {
    todo!()
}
