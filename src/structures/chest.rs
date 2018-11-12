use camera::Camera;
use collision::Collidable;
use color;
use common::AbsolutePosition;
use entity::Entity;
use filter;
use gfx;
use gfx::Canvas;
use gfx::Drawable;
use level;
use math::Rect;
use projectile::Projectile;
use resources;
use structures::Structure;
use InputState;

pub struct Chest {
    obj: Rect,

    hp: isize,
}

impl Chest {
    pub fn new(pos: AbsolutePosition) -> Self {
        Chest {
            obj: Rect {
                pos: pos.to_tuple(),
                size: (16f32, 16f32),
            },
            hp: 5,
        }
    }
}

impl Entity for Chest {
    fn update(&mut self, _input: &InputState, _delta: f32, _level: &level::Level) {}

    fn draw(&self, canvas: &mut Canvas, camera: &Camera) {
        let tileset_data = gfx::RGBData::new(resources::TILESET, resources::TILESET_SIZE);;
        let tileset = gfx::Tileset::new(&tileset_data, (16, 16));

        let tile = tileset.get_tile((1, 0)).unwrap();
        let img = filter::TransparencyFilter::new(&tile, color::PINK);
        let sprite = gfx::Sprite::new(&img);

        sprite.draw(self.position().to_screen(camera), canvas);
    }

    fn on_hit(&mut self, projectile: &Projectile, _level: &level::Level) {
        self.hp -= projectile.damage();
    }
}

impl Structure for Chest {
    fn destroy(&mut self, _level: &level::Level) {}
    fn is_alive(&self) -> bool {
        self.hp > 0
    }

    fn position(&self) -> AbsolutePosition {
        AbsolutePosition::from_f32(self.obj.pos)
    }
}

impl Collidable for Chest {
    fn hitbox(&self) -> Option<Rect> {
        Some(Rect {
            pos: (self.obj.pos.0 + 1f32, self.obj.pos.1 + 1f32),
            size: (13f32, 15f32),
        })
    }
}
