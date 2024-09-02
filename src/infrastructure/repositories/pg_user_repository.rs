use std::sync::Arc;
use async_trait::async_trait;
use diesel::prelude::*;
use diesel::{ExpressionMethods, OptionalExtension, RunQueryDsl};
use crate::domain::entities::user::{NewUser, User};
use crate::domain::repositories::user_repository::UserRepository;
use crate::infrastructure::database::connection::{establish_connection, DbPool};
use crate::infrastructure::schema::users;

#[derive(Clone)]
pub struct PgUserRepository {
    pool: DbPool,
}

impl PgUserRepository {
    pub fn new() -> Self {
        let database_url = std::env::var("DATABASE_URL")
            .expect("DATABASE_URL not found.");
        Self {
            pool: establish_connection(&database_url),
        }
    }
}

#[async_trait]
impl UserRepository for Arc<PgUserRepository> {
    async fn save(&self, new_user: &NewUser) -> Result<(), diesel::result::Error> {
        let conn = &mut self.pool.get().unwrap();
        diesel::insert_into(users::table)
            .values(new_user)
            .execute(conn)
            .expect("Error to insert User");
        Ok(())
    }

    async fn get_by_email(&self, email: String) -> Option<User> {
        let conn = &mut self.pool.get().unwrap();
        users::table
            .filter(users::email.eq(email))
            .first(conn)
            .optional()
            .expect("Error to find User")
    }
}
