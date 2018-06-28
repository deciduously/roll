use schema::items;
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
