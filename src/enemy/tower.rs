use alloc::boxed::Box;
use camera::Camera;
use collision::Collidable;
use color;
use common::AbsolutePosition;
use effects;
use enemy::Enemy;
use entity::Entity;
use filter;
use gfx;
use gfx::Canvas;
use gfx::Drawable;
use level;
use math::{normalize, Rect};
use misc::inside_vision;
use powerup;
use projectile;
use resources;
use InputState;

enum Status {
    Shooting,
    Idle,
}

pub struct Tower {
    obj: Rect,

    // action
    status: Status,
    ready: bool,
    time_until_ready: f32,

    // health related stuff
    hp: isize,
}

impl Tower {
    pub fn new(pos: (f32, f32), level: u32) -> Self {
        // Put some meaningful default values here.
        Self {
            obj: Rect {
                pos: pos,
                size: (16f32, 16f32),
            },

            status: Status::Idle,
            ready: false,
            time_until_ready: 1000f32,

            hp: 3 + level as isize / 2,
        }
    }

    pub fn get_center_position(&self) -> AbsolutePosition {
        AbsolutePosition::from_f32((
            self.obj.pos.0 + self.obj.size.0 / 2f32,
            self.obj.pos.1 + self.obj.size.1 / 2f32,
        ))
    }
}

impl Collidable for Tower {
    fn hitbox(&self) -> Option<Rect> {
        Some(self.obj.clone())
    }
}

impl Entity for Tower {
    fn update(&mut self, _input: &InputState, delta: f32, level: &level::Level) {
        if self.time_until_ready > 0f32 {
            self.time_until_ready -= delta;
        }
        match self.status {
            Status::Idle => {
                if self.time_until_ready <= 0f32 {
                    self.ready = true;
                }
            }
            Status::Shooting => {
                if self.time_until_ready <= 0f32 {
                    // shoot player
                    let center = self.get_center_position();
                    let target = level.player.borrow().get_center_position();
                    let vec = target.sub(center);
                    let vec = normalize((vec.0, vec.1));
                    // Scale normalized vector by the player graphics,
                    let vec = AbsolutePosition(vec.0, vec.1).mul(15f32);

                    &level.projectiles.borrow_mut().push(Box::new(
                        projectile::SimpleProjectile::new_from_absolute_coords(
                            &center.add(vec),
                            &target,
                            300f32,
                            2000f32,
                            projectile::Team::Villain,
                            color::ORANGE_YELLOW,
                        ),
                    ));

                    self.ready = true;
                }
            }
        }
        if self.ready {
            if inside_vision(
                &level.player.borrow().get_center_position(),
                &self.get_center_position(),
                &level.map.borrow(),
            ) {
                self.status = Status::Shooting;
            } else {
                self.status = Status::Idle;
            }
            self.time_until_ready = match self.status {
                Status::Idle => 500f32,
                Status::Shooting => 1000f32,
            };
            self.ready = false;
        }
    }

    fn draw(&self, canvas: &mut Canvas, camera: &Camera) {
        let screen_pos = self.position().to_screen(camera);
        let tileset_data = gfx::RGBData::new(resources::TILESET, resources::TILESET_SIZE);
        let tileset = gfx::Tileset::new(&tileset_data, (16, 16));

        // Draw background
        let bg_tile = tileset.get_tile((4, 1)).unwrap();
        let sprite = gfx::Sprite::new(&bg_tile);
        sprite.draw(screen_pos, canvas);

        // Draw enemy
        let tile = tileset.get_tile((5, 0)).unwrap();
        let img = filter::TransparencyFilter::new(&tile, color::PINK);
        let sprite = gfx::Sprite::new(&img);
        sprite.draw(screen_pos, canvas);

        // Draw weapon
        let tile = tileset.get_tile((6, 0)).unwrap();
        let img = filter::TransparencyFilter::new(&tile, color::PINK);
        let sprite = gfx::Sprite::new(&img);

        sprite.draw(screen_pos, canvas);
    }

    fn on_hit(&mut self, projectile: &projectile::Projectile, level: &level::Level) {
        self.hp -= projectile.damage();

        effects::on_hit_effect(
            projectile,
            level,
            self.get_center_position(),
            2,
            color::DARK_GREEN,
            color::GREEN,
        );
    }
}

impl Enemy for Tower {
    fn die(&mut self, level: &level::Level) {
        powerup::maybe_drop_powerup(self.position(), 10, level);
    }

    fn is_alive(&self) -> bool {
        self.hp > 0
    }

    fn position(&self) -> AbsolutePosition {
        AbsolutePosition::from_f32(self.obj.pos)
    }
}
