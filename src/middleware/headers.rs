use actix_web::middleware::DefaultHeaders;

pub fn default() -> DefaultHeaders {
    DefaultHeaders::new().add(("version", ""))
}

