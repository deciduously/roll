use diesel::prelude::*;
use r2d2;
use r2d2_diesel::ConnectionManager;
use std::ops::Deref;

lazy_static! {
    pub static ref DB_POOL: Pool = init_pool();
}

pub type Pool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

// all consts are 'static
pub const DATABASE_URL: &str = dotenv!("DATABASE_URL");

pub fn init_pool() -> Pool {
    //let config = r2d2::Config::default();
    let manager = ConnectionManager::<SqliteConnection>::new(DATABASE_URL);

    r2d2::Pool::new(manager).expect("failed to create pool")
}

pub struct Conn(r2d2::PooledConnection<ConnectionManager<SqliteConnection>>);

impl Deref for Conn {
    type Target = SqliteConnection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
