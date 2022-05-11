use actix_cors::Cors;

pub fn default() -> Cors {
    if cfg!(debug_assertions) {
        Cors::default()
            .send_wildcard()
            .allow_any_origin()
            .allow_any_header()
            .allow_any_method()
            .max_age(3600)
    } else {
        Cors::default()
    }
}

