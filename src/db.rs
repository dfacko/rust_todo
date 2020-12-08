pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;
use self::diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

#[macro_use]
use crate::diesel;

pub fn establish_connection() -> Pool {
    dotenv::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("Database not foudn");
    Pool::builder()
        .build(ConnectionManager::<PgConnection>::new(database_url))
        .unwrap()
}
