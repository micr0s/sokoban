use specs::World;

use crate::components::{Position, BoxColour};
use crate::entities::*;

pub fn load_map(world: &mut World, map_string: String) {
    let rows: Vec<&str> = map_string.trim().split('\n').map(|x| x.trim()).collect();
    for (y, row) in rows.iter().enumerate() {

        let cols: Vec<&str> = row.split(' ').collect();
        for (x, col) in cols.iter().enumerate() {
            let pos = Position {
                x: x as u8,
                y: y as u8,
                z: 0
            };
            match *col {
                "." => create_floor(world, pos),
                "W" => {
                    create_floor(world, pos);
                    create_wall(world, pos);
                },
                "P" => {
                    create_floor(world, pos);
                    create_player(world, pos);
                },
                "RB" => {
                    create_floor(world, pos);
                    create_box(world, pos, BoxColour::Red);
                },
                "BB" => {
                    create_floor(world, pos);
                    create_box(world, pos, BoxColour::Blue);
                },
                "RS" => {
                    create_floor(world, pos);
                    create_box_spot(world, pos, BoxColour::Red);
                },
                "BS" => {
                    create_floor(world, pos);
                    create_box_spot(world, pos, BoxColour::Blue);
                },
                "N" => (),
                c => panic!("unrecognized map item {}", c),
            }
        }
    }
}