use futures::future::{err, ok, Ready};
use uuid::Uuid;

use crate::domain::Session;
use crate::error::Error;
use crate::result::Result;
use actix_web::FromRequest;

pub struct LoginUser {
    pub id: Uuid,
    pub token: String,
}

impl FromRequest for LoginUser {
    type Error = Error;

    type Future = Ready<Result<Self>>;

    #[inline]
    fn from_request(
        req: &actix_web::HttpRequest,
        _payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        let authorization_header = req.headers().get("Authorization");
        match authorization_header {
            Some(authorization_header_value) => {
                let authorization = authorization_header_value.to_str();
                match authorization {
                    Ok(mut token) => {
                        if token.starts_with("bearer") {
                            token = &token[7..];
                        }
                        match Session::try_decode(token) {
                            Ok(session) => {
                                ok(LoginUser {
                                    id: session.user_id.clone(),
                                    token: token.to_string(),
                                })
                            }
                            Err(error) => err(error),
                        }
                    }
                    Err(_) => err(Error::AuthenticationFailedError),
                }
            }
            None => err(Error::AuthenticationFailedError),
        }
    }
}
