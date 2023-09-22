use std::env;
use diesel::{
    pg::PgConnection,
    prelude::*
};
use deadpool_diesel::{
    Pool,
    Manager,
    Error
};
use deadpool::managed::BuildError;
use dotenvy::dotenv;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn create_pool() -> Result<Pool<Manager<PgConnection>>, BuildError<Error>> {
    let db_url = std::env::var("DATABASE_URL").unwrap();

    // set up connection pool
    let manager = deadpool_diesel::postgres::Manager::new(db_url, deadpool_diesel::Runtime::Tokio1);
    let pool = deadpool_diesel::postgres::Pool::builder(manager)
        .build();

    return pool
}