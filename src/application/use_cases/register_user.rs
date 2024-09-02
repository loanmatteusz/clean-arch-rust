use crate::domain::entities::user::NewUser;
use crate::domain::repositories::user_repository::UserRepository;
use crate::domain::services::user_service::UserService;

pub struct RegisterUserUseCase<R: UserRepository> {
    service: UserService<R>,
}

impl <R: UserRepository> RegisterUserUseCase<R> {
    pub fn new(repository: R) -> Self {
        let service = UserService::new(repository);
        Self { service }
    }

    pub async fn execute(&self, new_user: NewUser) -> Result<(), diesel::result::Error> {
        self.service.register(new_user).await
    }
}
