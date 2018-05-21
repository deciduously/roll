use roll::*;

#[derive(Debug)]
pub struct Item {
    pub name: String,
    pub damage: Roll,
    pub level: u8,
}

impl Item {
    pub fn new(name: String, damage: &str, level: u8) -> Result<Item, String> {
        Ok(Item {
            name,
            damage: Roll::new(damage)?,
            level,
        })
    }
}
