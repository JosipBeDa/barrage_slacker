use diesel::{pg::PgConnection, r2d2::ConnectionManager, r2d2::Pool, Connection};
use dotenv::dotenv;
use std::env;

pub type DbPool = Pool<ConnectionManager<PgConnection>>;

/// Create connection pool for global application use
pub fn get_connection_pool() -> DbPool {
  dotenv().ok();
  let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
  let manager = ConnectionManager::<PgConnection>::new(database_url);

  Pool::builder()
    .build(manager)
    .expect("Failed to create database connection pool.")
}

/// Get single db connection instance when cannot use the pool
pub fn get_single_connection() -> PgConnection {
  dotenv().ok();
  let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

  PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}
