use super::*;
use crate::prelude::*;
use crate::spawner::template::Templates;

#[test]
fn test_data_load() {
    let loaded = Templates::load();
    println!("Loaded: {}", loaded.entities.len());
    loaded
        .entities
        .iter()
        .for_each(|e| println!("{} => Glyph: {}", e.name, e.glyph.chars().nth(0).unwrap()))
}
