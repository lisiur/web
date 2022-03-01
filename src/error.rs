use crate::response::JsonResponse;
use actix_web::ResponseError;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("SQL 执行报错: {0}")]
    SqlError(#[from] sqlx::Error),
    #[error("Jwt 错误: {0}")]
    JwtError(#[from] jsonwebtoken::errors::Error),
    #[error("认证失败")]
    AuthenticationFailedError,
    #[error("用户名已存在")]
    UserNameExistsError,
}

impl Error {
    pub fn get_code(&self) -> u16 {
        match self {
            Error::UserNameExistsError => 422,
            _ => 500,
        }
    }
}

impl ResponseError for Error {}
