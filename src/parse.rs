use item::{Item, Items};
use serde_yaml;
use std::{fs::File, io::{self, BufReader, prelude::*}};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RawItems {
    pub items: Vec<RawItem>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RawItem {
    pub name: String,
    pub damage: String,
}

fn rawitems_to_items(raw: &RawItems) -> io::Result<Items> {
    let mut ret = Items::new();
    for item in &raw.items {
        ret.insert(item.name.to_string(), Item::from(item.clone())?);
    }
    Ok(ret)
}

pub fn load_items() -> io::Result<Items> {
    let file = File::open("data/items.yaml")?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;

    let deserialized_items: RawItems = serde_yaml::from_str(&contents).unwrap();

    Ok(rawitems_to_items(&deserialized_items)?)
}
