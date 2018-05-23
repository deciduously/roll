use parse::RawItem;
use roll::*;
use std::{fmt, io, collections::HashMap};

#[derive(Debug, PartialEq)]
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

pub type Items = HashMap<String, Item>;
