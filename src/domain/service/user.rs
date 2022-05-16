use crate::domain::UserRepo;

pub struct UserService<'a>(&'a dyn UserRepo);

impl UserService<'_> {
    
}
