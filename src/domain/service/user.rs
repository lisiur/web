use crate::domain::repo::UserRepo;

pub struct UserService<'a>(&'a dyn UserRepo);

impl UserService<'_> {
    
}
