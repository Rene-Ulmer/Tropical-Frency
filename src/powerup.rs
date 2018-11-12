use camera;
use collision;
use common;
use entity;
use filter;
use gfx;
use gfx::Drawable;
use level;
use math;
use projectile;
use random;
use resources;
use InputState;

enum PowerupType {
    Health,
    PistolClip,
    RocketClip,
}

pub fn maybe_drop_powerup(position: common::AbsolutePosition, prob: u32, level: &level::Level) {
    if random::rand() % 101 <= prob {
        level
            .powerup
            .borrow_mut()
            .push(Powerup::new_random(position));
    }
}

pub struct Powerup {
    position: common::AbsolutePosition,
    poweruptype: PowerupType,
}

impl Powerup {
    pub fn new_random(position: common::AbsolutePosition) -> Self {
        let poweruptype = match random::rand_between(0, 10) {
            0..=6 => PowerupType::Health,
            7..=8 => PowerupType::PistolClip,
            9..=10 => PowerupType::RocketClip,
            _ => unreachable!(),
        };

        Self {
            position,
            poweruptype,
        }
    }

    pub fn on_pickup(&self, level: &level::Level) {
        let player = level.player.borrow_mut();
        match self.poweruptype {
            PowerupType::Health => {
                if player.hp < 20 {
                    player.hp += 1;
                }
            }
            PowerupType::PistolClip => {
                player.weapons[0].on_powerup_pickup();
            }
            PowerupType::RocketClip => {
                player.weapons[1].on_powerup_pickup();
            }
        }
    }
}

impl collision::Collidable for Powerup {
    fn hitbox(&self) -> Option<math::Rect> {
        let size = match self.poweruptype {
            PowerupType::Health => (7f32, 6f32),
            PowerupType::PistolClip => (5f32, 10f32),
            PowerupType::RocketClip => (6f32, 14f32),
        };

        Some(math::Rect {
            pos: self.position.to_tuple(),
            size,
        })
    }
}

impl entity::Entity for Powerup {
    fn update(&mut self, _input_state: &InputState, _delta: f32, _level: &level::Level) {}

    fn draw(&self, canvas: &mut gfx::Canvas, camera: &camera::Camera) {
        let tileset_data = gfx::RGBData::new(resources::TILESET, resources::TILESET_SIZE);;
        let tileset_data = filter::TransparencyFilter::new(&tileset_data, 0);
        let tileset = gfx::Tileset::new(&tileset_data, (16, 16));

        let p = self.position.to_screen(camera);
        let tile_offset = match self.poweruptype {
            PowerupType::Health => (0, 1),
            PowerupType::PistolClip => (1, 1),
            PowerupType::RocketClip => (3, 1),
        };

        let img = tileset.get_tile(tile_offset).unwrap();
        gfx::Sprite::new(&img).draw(p, canvas);
    }

    fn on_hit(&mut self, _projectile: &projectile::Projectile, _level: &level::Level) {}
}
