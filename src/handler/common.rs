use crate::error::Error;
use crate::response::JsonResponseSchema;
use actix_web::error::InternalError;
use actix_web::{Error as ActixError, HttpRequest, HttpResponse};
use std::fmt::Display;

pub fn handle_400<T: Display>(err: T, _req: &HttpRequest) -> ActixError {
    let error = Error::RequestError(err.to_string());
    let json = JsonResponseSchema::from_err(&error);
    InternalError::from_response(
        "",
        HttpResponse::BadRequest()
            .content_type("application/json; charset=utf-8")
            .body(serde_json::to_string(&json).unwrap()),
    )
    .into()
}
