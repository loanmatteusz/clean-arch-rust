use actix_web::{web, App, HttpServer};
use actix_web::middleware::Logger;
use log::info;
use crate::infrastructure::repositories::pg_user_repository::PgUserRepository;
use crate::presentation::routes;

pub async fn run() -> std::io::Result<()> {
    let _ = dotenv::dotenv();
    let repository = PgUserRepository::new();
    let app_data = web::Data::new(repository);

    info!("Starting...");

    HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            .wrap(Logger::default())
            .configure(routes::user_routes::routes)
    })
        .bind("0.0.0.0:4800")
        .unwrap()
        .run()
        .await
}
