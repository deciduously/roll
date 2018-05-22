use item::Item;
use serde_yaml;
use std::{fs::File, io::{self, BufReader, prelude::*}};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RawItem {
    pub name: String,
    pub damage: String,
}

pub fn load_item() -> io::Result<Item> {
    let mut file = File::open("data/blello.yaml")?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents);

    let deserialized_item: RawItem = serde_yaml::from_str(&contents).unwrap();
    Ok(Item::new(deserialized_item)?)
}
