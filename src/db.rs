use diesel::prelude::*;
use models::*;
use item::Items;
use roll::Roll;
use std::env;

pub fn establish_connection() -> SqliteConnection {
    let db_url = dotenv!("DATABASE_URL"); // TODO if not set?

    SqliteConnection::establish(&db_url).expect(&format!("Error connecting to {}", db_url))
}
