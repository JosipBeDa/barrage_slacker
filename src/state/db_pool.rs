use diesel::r2d2::{ConnectionManager, Pool};
use diesel::{pg::PgConnection, Connection};
use dotenv::dotenv;
use std::env;

pub type PgPool = Pool<ConnectionManager<PgConnection>>;

// Create connection pool for global application use
pub fn establish_pool_connection() -> PgPool {
  dotenv().ok();
  let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
  let manager = ConnectionManager::<PgConnection>::new(database_url);
  // min_idle VERY IMPORTANT for sanity, if not set the server will use the start with YOLO method
  Pool::builder()
    .min_idle(Some(1))
    .build(manager)
    .expect("Failed to create pool.")
}

// Get single db connection instance when cannot use the pool
#[allow(dead_code)]
pub fn establish_connection() -> PgConnection {
  dotenv().ok();
  let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

  PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}
