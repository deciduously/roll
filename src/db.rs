use diesel::prelude::*;
use models::*;
use std::env;

pub fn establish_connection() -> SqliteConnection {
    let db_url = dotenv!("DATABASE_URL"); // TODO if not set?

    SqliteConnection::establish(&db_url).expect(&format!("Error connecting to {}", db_url))
}

pub fn show_items() {
    use schema::items::dsl::*;

    let connection = establish_connection();
    let results = items.limit(5).load::<Item>(&connection).expect("Error loading items");

    println!("Displaying {} items", results.len());
    for item in results {
        println!("{}\n----------\n{}", item.title, item.damage);
    }
}
