use diesel::PgConnection;
use diesel::r2d2::ConnectionManager;

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn establish_connection(db_url: &str) -> DbPool {
    let manager = ConnectionManager::<PgConnection>::new(db_url);
    r2d2::Pool::builder()
        .build(manager)
        .expect("Failure to create Pool.")
}
