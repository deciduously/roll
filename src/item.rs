use actix_web::{Responder, HttpRequest, HttpResponse, Error};
use db::DB_POOL;
use diesel::{self, prelude::*};
use models::*;
use roll::*;
use serde_json;
use std::{collections::HashMap, io};

//pub type Items = HashMap<String, Roll>; // TODO remove if unnecessary?

// I think i want {:name blah :damage blah}

pub fn create_item<'a>(conn: &SqliteConnection, title: &'a str, damage: &'a str) -> usize {
    use schema::items;

    let new_item = NewItem { title, damage };

    diesel::insert_into(items::table)
        .values(&new_item)
        .execute(conn)
        .expect("Error saving new item")
}

pub fn get_items(conn: &SqliteConnection) -> Items {
    use schema::items::dsl::*;
    let results = items
        .limit(5)
        .load::<Item>(conn)
        .expect("Error loading items");

    println!("Displaying {} items", results.len());
    let mut ret = Vec::new();
    for item in results {
        println!("{}\n----------\n{}", item.title, item.damage);
        ret.push(item);
    }
    Items{ items: ret}
}

#[derive(Debug, Serialize)]
pub struct Items {
    pub items: Vec<Item>,
}

impl Responder for Items {
    type Item = HttpResponse;
    type Error = Error;

    fn respond_to<S>(self, _req: &HttpRequest<S>) -> Result<HttpResponse, Error> {
        let body = serde_json::to_string(&self)?;

        Ok(HttpResponse::Ok().content_type("application/json").body(body))
    }
}

//pub fn get_item_by_name(conn: &SqliteConnection, name: String) -> io::Result<String> {
//    use schema::items::{self, dsl::*};
//
//    items::table.select(name).load::<String>(conn)?
//}
