use std::path;

use ggez;
use ggez::{conf, Context, event, GameResult};
use ggez::event::{KeyCode, KeyMods};
use specs::{RunNow, World, WorldExt};

mod resources;
mod map;
mod entities;
mod constants;
mod components;
mod systems;

// This struct will hold all our game state
// For now there is nothing to be held, but we'll add
// things shortly.
struct Game {
    world: World,
}

// This is the main event loop. ggez tells us to implement
// two things:
// - updating
// - rendering
impl event::EventHandler for Game {
    fn update(&mut self, _context: &mut Context) -> GameResult {
        // Run input system
        {
            let mut is = systems::InputSystem {};
            is.run_now(&self.world);
        }
        Ok(())
    }

    fn key_down_event(
        &mut self,
        _context: &mut Context,
        keycode: KeyCode,
        _keymod: KeyMods,
        _repeat: bool,
    ) {
        let mut input_queue = self.world.write_resource::<resources::InputQueue>();
        input_queue.keys_pressed.push(keycode);
    }

    fn draw(&mut self, context: &mut Context) -> GameResult {
        // Render game entities
        {
            let mut rs = systems::RenderingSystem { context };
            rs.run_now(&self.world);
        }

        Ok(())
    }
}

//todo: load from file
// Initialize the level
pub fn initialize_level(world: &mut World) {
    const MAP: &str = "
    N N W W W W W W
    W W W . . . . W
    W . . . B . . W
    W . . . . . . W
    W . P . . . . W
    W . . . . . . W
    W . . S . . . W
    W . . . . . . W
    W W W W W W W W
    ";

    map::load_map(world, MAP.to_string());
}

pub fn main() -> GameResult {
    let mut world = World::new();
    components::register_components(&mut world);
    resources::register_resources(&mut world);
    initialize_level(&mut world);

    // Create a game context and event loop
    let context_builder = ggez::ContextBuilder::new("rust_sokoban", "sokoban")
        .window_setup(conf::WindowSetup::default().title("Rust Sokoban!"))
        .window_mode(conf::WindowMode::default().dimensions(800.0, 600.0))
        .add_resource_path(path::PathBuf::from("./resources"));

    let (context, event_loop) = &mut context_builder.build()?;

    // Create the game state
    let game = &mut Game { world };
    // Run the main event loop
    event::run(context, event_loop, game)
}