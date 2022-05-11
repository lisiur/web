use crate::response::JsonResponseSchema;
use actix_web::{
    http::{header::ContentType, StatusCode},
    HttpResponse, ResponseError,
};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("请求参数错误: {0}")]
    RequestError(String),
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
    pub fn get_status_code(&self) -> StatusCode {
        match self {
            Error::RequestError(_) => StatusCode::BAD_REQUEST,
            Error::UserNameExistsError => StatusCode::UNPROCESSABLE_ENTITY,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    pub fn get_biz_code(&self) -> u16 {
        match self {
            Error::RequestError(_) => 400,
            Error::UserNameExistsError => 4_001,
            _ => 5_000,
        }
    }
}

impl ResponseError for Error {
    fn status_code(&self) -> actix_web::http::StatusCode {
        self.get_status_code()
    }

    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        let error = JsonResponseSchema::from_err(self);
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .body(serde_json::to_string(&error).unwrap())
    }
}
