use std::fs;
use std::collections::HashMap;
use specs::{World, WorldExt};
use ggez::Context;
use crate::map;

#[derive(Default)]
pub struct LevelStore {
    pub levels: HashMap<String, String>,
}

impl LevelStore {
    pub fn load_level(&mut self, level: u8) {
        let level_code = format!("{:02}", level);
        let level_source = self
            .levels
            .get_mut(&level_code[..])
            .expect("expected level");
        map::load_map(world, String.from(level_source));
    }
}

pub fn initialize_levels(world: &mut World) {
    let mut level_store = world.write_storage::<LevelStore>();
    let levels = ["00", "01"];

    for level in levels.iter() {
        let level_code = format!("{}", level.to_string());
        let level_path = format!("/levels/level_{}", level_code);
        let level_source =  fs::read_to_string(level_path).expect("expected level loaded");

        level_store.levels.insert(level_code, level_source)
    }
}