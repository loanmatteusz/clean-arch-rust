use actix_web::{get, post, web, HttpResponse};
use actix_web::web::{Json, Path};
use log::error;
use crate::application::use_cases::get_user::GetUserUseCase;
use crate::application::use_cases::register_user::RegisterUserUseCase;
use crate::domain::entities::user::NewUser;
use crate::infrastructure::repositories::pg_user_repository::PgUserRepository;

#[post("/")]
pub async fn register_user_handler(
    repository: web::Data<PgUserRepository>,
    new_user: Json<NewUser>
) -> HttpResponse {
    let register_user_use_case = RegisterUserUseCase::new(repository.into_inner());
    match register_user_use_case.execute(new_user.into_inner()).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            error!("Error registring user: {:?}", e);
            eprintln!("{}", e);
            HttpResponse::InternalServerError().body("Please try again...!")
        }
    }
}


#[get("/{email}")]
pub async fn get_user_handler(
    repository: web::Data<PgUserRepository>,
    path: Path<(String,)>
) -> HttpResponse {
    let get_user_use_case = GetUserUseCase::new(repository.into_inner());
    match get_user_use_case.get(path.into_inner().0).await {
        Some(user) => HttpResponse::Ok().json(user),
        None => {
            error!("Error to get user!");
            HttpResponse::InternalServerError().body("Please try again...!")
        }
    }
}
