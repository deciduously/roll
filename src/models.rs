use actix_web::{Error, HttpRequest, HttpResponse, Responder};
// use diesel::prelude::Identifiable;
use schema::items;
use serde_json;
use std::fmt;

#[derive(Debug, Queryable, Serialize)]
pub struct Item {
    pub id: i32,
    pub title: String,
    pub damage: String,
}

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} - {} damage", self.title, self.damage)
    }
}

#[derive(Debug, Deserialize, Insertable)]
#[table_name = "items"]
pub struct NewItem<'a> {
    pub title: &'a str,
    pub damage: &'a str,
}

#[derive(Debug, Deserialize)]
pub struct RequestItem {
    pub name: String,
    pub damage: String,
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

        Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(body))
    }
}
