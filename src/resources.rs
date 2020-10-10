use ggez::event::KeyCode;
use specs::World;

#[derive(Default)]
pub struct InputQueue {
    pub keys_pressed: Vec<KeyCode>,
}

// Registering resources
pub fn register_resources(world: &mut World) {
    world.insert(InputQueue::default())
}