use env_logger::Env;
use crate::infrastructure::web::run;

mod application;
mod domain;
mod infrastructure;
mod presentation;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    run().await
}
