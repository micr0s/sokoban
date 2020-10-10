use ggez::{Context, graphics};
use ggez::graphics::{DrawParam, Image, Color};
use ggez::nalgebra as na;
use specs::{Join, ReadStorage, System, Read};

use crate::components::{Position, Renderable, RenderableKind};
use crate::constants::{TILE_SIZE, MAP_WIDTH, STATE_DLMR_WIDTH, STATE_DLMR_HEIGHT};
use crate::resources::{Gameplay, Time};
use std::time::Duration;

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

    pub fn get_image(&mut self, renderable: &Renderable, delta: Duration) -> Image {
        let path_index = match renderable.kind() {
            RenderableKind::Static => {
                0
            }
            RenderableKind::Animated => {
                // If we have multiple, we want to select the right one based on the delta time.
                // First we get the delta in milliseconds, we % by 1000 to get the milliseconds
                // only and finally we divide by 250 to get a number between 0 and 4. If it's 4
                // we technically are on the next iteration of the loop (or on 0), but we will let
                // the renderable handle this logic of wrapping frames.
                ((delta.as_millis() % 1000) / 250) as usize
            }
        };
        let image_path = renderable.path(path_index);

        Image::new(self.context, image_path).expect("expected image")
    }
}

// System implementation
impl<'a> System<'a> for RenderingSystem<'a> {
    // Data
    type SystemData = (Read<'a, Gameplay>, Read<'a, Time>, ReadStorage<'a, Position>, ReadStorage<'a, Renderable>);

    fn run(&mut self, data: Self::SystemData) {
        let (gameplay, time, positions, renderables) = data;

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
            let image = self.get_image(renderable, time.delta);
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