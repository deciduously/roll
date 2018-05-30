use diesel::prelude::*;
use std::env;

pub fn establish_connection() -> SqliteConnection {
    let db_url = dotenv!("DATABASE_URL"); // TODO if not set?

    SqliteConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}
