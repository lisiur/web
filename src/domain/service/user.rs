use crate::domain::{User, UserRepo};
use crate::error::Error;
use crate::result::Result;
use uuid::Uuid;

pub struct UserInfoService<'a> {
    user_repo: &'a dyn UserRepo,
}

impl<'a> UserInfoService<'a> {
    pub fn new(user_repo: &'a dyn UserRepo) -> Self {
        Self { user_repo }
    }

    pub async fn exec(&self, user_id: &Uuid) -> Result<User> {
        let user = self.user_repo.find_by_id(&user_id.to_string()).await?;
        match user {
            Some(user) => Ok(user),
            None => Err(Error::UserNotExistsError),
        }
    }
}
