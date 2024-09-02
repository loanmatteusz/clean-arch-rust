use async_trait::async_trait;
use crate::domain::entities::user::{NewUser, User};

#[async_trait]
pub trait UserRepository {
    async fn save(&self, new_user: &NewUser) -> Result<(), diesel::result::Error>;
    async fn get_by_email(&self, email: String) -> Option<User>;
}
