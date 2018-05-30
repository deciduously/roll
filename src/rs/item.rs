use parse::RawItem;
use roll::*;
use std::{collections::HashMap, fmt, io};

#[derive(Debug, PartialEq, Serialize)]
pub struct Item {
    pub name: String,
    pub damage: Roll,
}

impl Item {
    pub fn from(raw: RawItem) -> io::Result<Item> {
        Ok(Item {
            name: raw.name,
            damage: Roll::new(&raw.damage)?,
        })
    }
}

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} - {} damage", self.name, self.damage)
    }
}

pub type Items = HashMap<String, Roll>;

pub fn lookup_item(name: &str, items: &Items) -> io::Result<(String, String)> {
    Ok((name.to_string(), items[name].to_string()))
}
