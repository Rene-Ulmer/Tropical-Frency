use common::AbsolutePosition;

use HEIGHT;
use WIDTH;

// TODO: Shaking etc.
pub struct Camera {
    position: AbsolutePosition,
}

impl Camera {
    pub fn new() -> Self {
        Self {
            position: AbsolutePosition(0f32, 0f32),
        }
    }

    pub fn get(&self) -> AbsolutePosition {
        self.position
    }

    pub fn update(&mut self, _delta: f32) {}

    pub fn center_around(&mut self, position: AbsolutePosition) {
        self.position.0 = position.0 - WIDTH as f32 / 2f32;
        self.position.1 = position.1 - HEIGHT as f32 / 2f32;
    }
}
