use crate::state::client;
use crate::state::db_pool;
use diesel::r2d2;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub client: reqwest::Client,
    pub db_pool: db_pool::DbPool,
}

pub type DbConnection = r2d2::PooledConnection<r2d2::ConnectionManager<diesel::PgConnection>>;

pub fn initialize() -> AppState {
    let db_pool = db_pool::get_connection_pool();
    let client = client::initialize();
    AppState { client, db_pool }
}
