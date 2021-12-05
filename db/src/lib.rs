#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
#[macro_use]
extern crate diesel;

pub mod models;
pub mod schema;

use std::env;

use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};

pub type PgPool = Pool<ConnectionManager<PgConnection>>;

pub fn get_conn(pool: &PgPool) -> PooledConnection<ConnectionManager<PgConnection>> {
    pool.get().unwrap()
}

pub fn new_pool() -> PgPool {
    // again using our environment variable
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);

    Pool::builder()
        .build(manager)
        .expect("failed to create db pool")
}
