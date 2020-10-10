use ggez::{Context, graphics};
use ggez::graphics::{DrawParam, Image, Color};
use ggez::nalgebra as na;
use specs::{Join, ReadStorage, System, Read};

use crate::components::{Position, Renderable};
use crate::constants::{TILE_SIZE, MAP_WIDTH, STATE_DLMR_WIDTH, STATE_DLMR_HEIGHT};
use crate::resources::Gameplay;

pub struct RenderingSystem<'a> {
    pub context: &'a mut Context,
}

impl RenderingSystem<'_> {
    pub fn draw_text(&mut self, text_string: &str, line: u8) {
        let text_from_x_tile = MAP_WIDTH + STATE_DLMR_WIDTH;
        let text_from_y_tile = STATE_DLMR_HEIGHT + line;
        let x = text_from_x_tile as f32 * TILE_SIZE;
        let y = text_from_y_tile as f32 * TILE_SIZE;

        let text = graphics::Text::new(text_string);
        let destination = na::Point2::new(x, y);
        let color = Some(Color::new(0.0, 0.0, 0.0, 1.0));
        let dimensions = na::Point2::new(0.0, 20.0);

        graphics::queue_text(self.context, &text, dimensions, color);
        graphics::draw_queued_text(
            self.context,
            graphics::DrawParam::new().dest(destination),
            None,
            graphics::FilterMode::Linear,
        )
            .expect("expected drawing queued text");
    }
}

// System implementation
impl<'a> System<'a> for RenderingSystem<'a> {
    // Data
    type SystemData = (Read<'a, Gameplay>, ReadStorage<'a, Position>, ReadStorage<'a, Renderable>);

    fn run(&mut self, data: Self::SystemData) {
        let (gameplay, positions, renderables) = data;

        // Clearing the screen (this gives us the backround colour)
        graphics::clear(self.context, graphics::Color::new(0.95, 0.95, 0.95, 1.0));

        // Get all the renderables with their positions and sort by the position z
        // This will allow us to have entities layered visually.
        let mut rendering_data = (&positions, &renderables).join().collect::<Vec<_>>();
        rendering_data.sort_by_key(|&k| k.0.z);

        // Iterate through all pairs of positions & renderables, load the image
        // and draw it at the specified position.
        for (position, renderable) in rendering_data.iter() {
            // Load the image
            let image = Image::new(self.context, renderable.path.clone()).expect("expected image");
            let x = position.x as f32 * TILE_SIZE;
            let y = position.y as f32 * TILE_SIZE;

            // draw
            let draw_params = DrawParam::new().dest(na::Point2::new(x, y));
            graphics::draw(self.context, &image, draw_params).expect("expected render");
        }

        // Render any text
        self.draw_text(&format!("State: {}", gameplay.state), 0);
        self.draw_text(&format!("Moves: {}", gameplay.moves_count), 1);

        // Finally, present the context, this will actually display everything
        // on the screen.
        graphics::present(self.context).expect("expected to present");
    }
}