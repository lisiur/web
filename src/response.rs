use crate::result::Result;
use actix_web::web::Json;
use serde::Serialize;
use crate::error::Error;

pub struct Response;

impl Response {
    pub fn json<T: Serialize>(data: T) -> JsonResponse<T> {
        Json(JsonResponseSchema::from_data(data))
    }
    pub fn json_ok<T: Serialize>(data: T) -> JsonResponseResult<T> {
        Ok(Json(JsonResponseSchema::from_data(data)))
    }
    #[allow(dead_code)]
    pub fn json_err(err: Error) -> JsonResponseResult<()> {
        Err(err)
    }
}

pub type JsonResponse<T> = Json<JsonResponseSchema<T>>;

type JsonResponseResult<T> = Result<Json<JsonResponseSchema<T>>>;

#[derive(Serialize)]
pub struct JsonResponseSchema<T: Serialize> {
    pub code: u16,
    pub data: Option<T>,
    pub message: Option<String>,
}

impl<T: Serialize> JsonResponseSchema<T> {
    pub fn from_data(data: T) -> JsonResponseSchema<T> {
        Self {
            code: 200,
            data: Some(data),
            message: None,
        }
    }
}

impl JsonResponseSchema<()> {
    pub fn from_err(err: &Error) -> JsonResponseSchema<()> {
        Self {
            code: err.get_biz_code(),
            data: None,
            message: Some(err.to_string()),
        }
    }
}

