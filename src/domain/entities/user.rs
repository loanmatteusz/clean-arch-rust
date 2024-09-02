use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::infrastructure::schema::users;

#[derive(Queryable, Serialize, Deserialize, Debug, Clone)]
// #[diesel(table_name = users)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub phone: String,
    pub address: String,
}

#[derive(Debug, Clone, Insertable, Deserialize)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub name: String,
    pub email: String,
    pub phone: String,
    pub address: String,
}
