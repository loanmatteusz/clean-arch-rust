use crate::domain::entities::user::{NewUser, User};
use crate::domain::repositories::user_repository::UserRepository;

#[derive(Clone)]
pub struct UserService<R: UserRepository> {
    repository: R,
}

impl<R: UserRepository> UserService<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn register(&self, new_user: NewUser) -> Result<(), diesel::result::Error> {
        self.repository.save(&new_user).await?;
        Ok(())
    }
    pub async fn get_by_email(&self, email: String) -> Option<User> {
        self.repository.get_by_email(email).await
    }
}
