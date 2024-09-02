use crate::domain::entities::user::User;
use crate::domain::repositories::user_repository::UserRepository;
use crate::domain::services::user_service::UserService;

pub struct GetUserUseCase<R: UserRepository> {
    service: UserService<R>,
}

impl <R: UserRepository> GetUserUseCase<R> {
    pub fn new(repository: R) -> Self {
        let service = UserService::new(repository);
        Self { service }
    }

    pub async fn get(&self, email: String) -> Option<User> {
        self.service.get_by_email(email).await
    }
}
