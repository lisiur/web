use crate::result::Result;
use actix_web::http::StatusCode;
use actix_web::{HttpRequest, HttpResponse, Responder};
use serde::Serialize;

pub type JsonResponseResult<T> = Result<JsonResponse<T>>;

#[derive(Serialize)]
pub struct JsonResponse<T: Serialize> {
    pub code: u16,
    pub data: T,
    pub message: Option<String>,
}

impl<T: Serialize> JsonResponse<T> {
    pub fn ok(data: T) -> JsonResponseResult<T> {
        Ok(Self {
            code: 200,
            data,
            message: None,
        })
    }
}

impl<T: Serialize> Responder for JsonResponse<T> {
    type Body = String;

    fn respond_to(self, req: &HttpRequest) -> HttpResponse<Self::Body> {
        HttpResponse::new(StatusCode::OK).set_body(serde_json::to_string(&self).unwrap())
    }
}
