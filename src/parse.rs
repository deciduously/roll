use item::Item;
use serde_yaml;
use std::{fs::File, io::{self, BufReader, prelude::*}};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RawItems {
    pub items: Vec<RawItem>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RawItem {
    pub name: String,
    pub damage: String,
}

pub fn load_items() -> io::Result<Vec<Item>> {
    let file = File::open("data/items.yaml")?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;

    let deserialized_items: RawItems = serde_yaml::from_str(&contents).unwrap();

    let mut ret = Vec::new();
    for i in deserialized_items.items {
        ret.push(Item::new(i)?);
    }

    Ok(ret)
}
