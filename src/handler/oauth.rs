use crate::domain::{Session, Oauth, User};
use crate::domain::{SessionDb, OauthDb, UserDb};
use crate::domain::{LoginByPasswordService, RegisterByPasswordService};
use crate::prelude::*;
use crate::response::{JsonResponse, Response};

/// 注册参数
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RegisterParams {
    /// 用户名
    username: String,
    /// 密码
    password: String,
    /// 邮箱
    email: String,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UserToken {
    pub id: Uuid,
    pub name: String,
    pub expired_at: i64,
    pub token: String,
}

/// 注册
#[post("/register")]
pub async fn register(
    pool: Data<DbPool>,
    params: Json<RegisterParams>,
    user_repo: Db<UserDb>,
    session_repo: Db<SessionDb>,
) -> Result<JsonResponse<UserToken>> {
    let pool = (**pool).clone();
    let session = RegisterByPasswordService::new(&*user_repo, &*session_repo)
        .exec(&params.username, &params.password, &params.email)
        .await?;

    Ok(Response::json(UserToken {
        id: session.user_id,
        name: session.user_name,
        expired_at: session.expired_at,
        token: session.token,
    }))
}

/// 登录参数
#[derive(Deserialize, Clone, Debug)]
pub struct LoginParams {
    /// 用户名
    username: String,
    /// 密码
    password: String,
}

/// 登录
#[utoipa::path]
#[post("/login")]
pub async fn login(
    pool: Data<DbPool>,
    params: Json<LoginParams>,
    user_repo: Db<UserDb>,
    session_repo: Db<SessionDb>,
    oauth_repo: Db<OauthDb>,
) -> Result<JsonResponse<UserToken>> {
    let pool = (**pool).clone();
    let session = LoginByPasswordService::new(&*user_repo, &*session_repo, &*oauth_repo)
        .exec(&params.username, &params.password)
        .await?;

    Ok(Response::json(UserToken {
        id: session.user_id,
        name: session.user_name,
        expired_at: session.expired_at,
        token: session.token,
    }))
}

/// 登出
pub async fn logout(pool: Data<DbPool>) -> Result<()> {
    todo!()
}
