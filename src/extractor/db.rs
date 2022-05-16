use crate::result::Result;
use crate::{error::Error, prelude::DbPool};
use actix_web::{web::Data, FromRequest};
use futures::future::{err, ok, Ready};
use sqlx::{Pool, Postgres};
use std::ops::Deref;

pub struct Db<T: From<Pool<Postgres>>>(pub T);

impl<T: From<Pool<Postgres>>> Deref for Db<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: From<Pool<Postgres>>> FromRequest for Db<T> {
    type Error = Error;

    type Future = Ready<Result<Self>>;

    fn from_request(
        req: &actix_web::HttpRequest,
        payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        match req.app_data::<Data<DbPool>>() {
            Some(pool) => {
                let pool = (***pool).clone();
                let db = Db(T::from(pool));
                ok(db)
            }
            None => unreachable!(),
        }
    }
}
