use camera::Camera;
use map;
use math;
pub type PositionComponent = f32;

#[derive(Copy, Clone)]
pub struct AbsolutePosition(pub PositionComponent, pub PositionComponent);

impl AbsolutePosition {
    pub fn to_tuple(&self) -> (f32, f32) {
        (self.0, self.1)
    }

    pub fn to_screen(&self, camera: &Camera) -> ScreenPosition {
        let screen_pos = self.sub(camera.get());
        ScreenPosition(screen_pos.0 as isize, screen_pos.1 as isize)
    }

    pub fn to_map(&self) -> MapPosition {
        MapPosition(
            (self.0 / map::TILE_WIDTH as f32) as isize,
            (self.1 / map::TILE_HEIGHT as f32) as isize,
        )
    }

    pub fn add(&self, other: AbsolutePosition) -> AbsolutePosition {
        AbsolutePosition(self.0 + other.0, self.1 + other.1)
    }

    pub fn sub(&self, other: AbsolutePosition) -> AbsolutePosition {
        AbsolutePosition(self.0 - other.0, self.1 - other.1)
    }

    pub fn mul(&self, factor: PositionComponent) -> AbsolutePosition {
        AbsolutePosition(self.0 * factor, self.1 * factor)
    }

    pub fn round(&self) -> AbsolutePosition {
        AbsolutePosition(math::round(self.0), math::round(self.1))
    }

    pub fn from_f32(values: (f32, f32)) -> Self {
        AbsolutePosition(values.0, values.1)
    }

    pub fn floor(&self) -> AbsolutePosition {
        AbsolutePosition(self.0 as isize as f32, self.1 as isize as f32)
    }
}

#[derive(Copy, Clone)]
pub struct ScreenPosition(pub isize, pub isize);

impl ScreenPosition {
    pub fn to_absolute(&self, offset: Option<AbsolutePosition>) -> AbsolutePosition {
        let delta = match offset {
            Some(d) => d,
            None => AbsolutePosition(0f32, 0f32),
        };

        let our = AbsolutePosition(self.0 as f32, self.1 as f32);
        our.add(delta)
    }
}

/// Represents the tile index in the map. Equals to absolute position
/// divided by the size of the tiles.
#[derive(Copy, Clone)]
pub struct MapPosition(pub isize, pub isize);

impl MapPosition {
    pub fn to_absolute(&self) -> AbsolutePosition {
        AbsolutePosition(
            (self.0 * map::TILE_WIDTH) as f32,
            (self.1 * map::TILE_HEIGHT) as f32,
        )
    }

    pub fn add(&self, other: &MapPosition) -> MapPosition {
        MapPosition(self.0 + other.0, self.1 + other.1)
    }

    pub fn sub(&self, other: &MapPosition) -> MapPosition {
        MapPosition(self.0 - other.0, self.1 - other.1)
    }
}

pub fn apply_bounds(val: f32, min: f32, max: f32) -> f32 {
    if val < min {
        min
    } else if val > max {
        max
    } else {
        val
    }
}
