use std::collections::HashMap;

use specs::{Join, ReadStorage, System, Write};

use crate::components::{Box, BoxSpot, Position};
use crate::resources::{Gameplay, GameplayState, EventQueue};
use crate::events::Event;

pub struct GameplayStateSystem {}

impl<'a> System<'a> for GameplayStateSystem {
    // Data
    type SystemData = (
        Write<'a, EventQueue>,
        Write<'a, Gameplay>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Box>,
        ReadStorage<'a, BoxSpot>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut events,
            mut gameplay_state,
            positions,
            boxes,
            box_spots) = data;

        // get all boxes indexed by position
        let boxes_by_position: HashMap<(u8, u8), &Box> = (&positions, &boxes)
            .join()
            .map(|t| ((t.0.x, t.0.y), t.1))
            .collect::<HashMap<_, _>>();

        // loop through all box spots and check if there is a corresponding
        // box at that position
        for (box_spot, position) in (&box_spots, &positions).join() {
            match boxes_by_position.get(&(position.x, position.y)) {
                Some(cur_box) => if cur_box.colour == box_spot.colour { continue; } else { return; },
                None => {
                    gameplay_state.state = GameplayState::Playing;
                    return;
                }
            }
        }

        // If we made it this far, then all box spots have boxes on them, and the
        // game has been won
        if gameplay_state.state != GameplayState::Won {
            gameplay_state.state = GameplayState::Won;
            events.events.push(Event::PlayerWon {});
            println!("You won in {} moves", gameplay_state.moves_count)
        }
    }
}