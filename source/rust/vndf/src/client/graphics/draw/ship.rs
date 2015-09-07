use nalgebra::{
    cast,

    Norm,
    ToHomogeneous,

    Iso3,
    Vec2,
    Vec3,
};

use client::graphics::base::{
    color,

    Graphics,
};
use client::graphics::draw::{
    GlyphDrawer,
    ShapeDrawer,
};
use client::graphics::frame_state::FrameState;
use client::interface::Frame;
use shared::util::angle_of;


pub struct ShipDrawer {
    ship_size     : f32,
    line_height   : f32,
    scaling_factor: f32,

    symbol_drawer: ShapeDrawer,
    glyph_drawer : GlyphDrawer,
    line_drawer  : ShapeDrawer,
}

impl ShipDrawer {
    pub fn new(
        graphics      : &mut Graphics,
        ship_size     : f32,
        font_size     : f32,
        scaling_factor: f32,
    ) -> ShipDrawer {
        ShipDrawer {
            ship_size     : ship_size,
            line_height   : font_size,
            scaling_factor: scaling_factor,

            symbol_drawer: ShapeDrawer::ship(graphics),
            glyph_drawer : GlyphDrawer::new(graphics, font_size as u32),
            line_drawer  : ShapeDrawer::line(graphics),
        }
    }

    pub fn draw(&mut self, frame: &Frame, frame_state: &mut FrameState) {
        for (ship_id, ship) in &frame.ships {
            let pos_offset    = Vec2::new(0.7, 0.3) * self.ship_size;
            let line_advance  = Vec2::new(0.0, -self.line_height);

            let ship_velocity: Vec2<f32> = cast(ship.velocity);

            let transform = frame_state.transforms.symbol_to_screen(cast(ship.position));

            // draw ship velocity line
            let line_rotation = Iso3::new(
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(
                    0.0,
                    0.0,
                    angle_of(ship_velocity),
                ),
            );
            self.line_drawer.draw(
                ship_velocity.norm() * self.scaling_factor * 50.0,
                color::Colors::red(),
                transform * line_rotation.to_homogeneous(),
                &mut frame_state.graphics,
            );

            if frame.select_ids.contains(ship_id) {
                self.symbol_drawer.draw(
                    self.ship_size * 1.25,
                    color::Colors::white(),
                    transform,
                    &mut frame_state.graphics,
                );
            }

            let mut color = color::Colors::blue();
            if let Some(sid) = frame.ship_id {
                if *ship_id == sid  { color = color::Colors::green_spring(); }
            }

            self.symbol_drawer.draw(
                self.ship_size,
                color,
                transform,
                &mut frame_state.graphics,
            );

            // draw ship id
            self.glyph_drawer.draw(
                &ship_id.to_string(),
                Vec2::new(0.0, self.ship_size * 0.6),
                color::Colors::white(),
                true,
                transform,
                &mut frame_state.graphics,
            );

            // draw ship broadcast
            if let Some(ship_comm) = frame.broadcasts.get(&ship_id) {
                self.glyph_drawer.draw(
                    ship_comm,
                    -Vec2::new(0.0, self.ship_size),
                    color::Colors::white(),
                    true,
                    transform,
                    &mut frame_state.graphics,
                );
            }

            // draw ship position
            let pos = format!("pos: ({:.2}, {:.2})", ship.position[0], ship.position[1]);
            self.glyph_drawer.draw(
                &pos,
                pos_offset,
                color::Colors::white(),
                false,
                transform,
                &mut frame_state.graphics,
            );

            // draw ship velocity
            let vel = format!("vel: ({:.2}, {:.2})", ship.velocity[0], ship.velocity[1]);
            self.glyph_drawer.draw(
                &vel,
                pos_offset + line_advance,
                color::Colors::white(),
                false,
                transform,
                &mut frame_state.graphics,
            );
        }
    }
}