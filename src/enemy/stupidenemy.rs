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
use math;
use math::{normalize, Rect};
use misc::inside_vision;
use movement;
use powerup;
use projectile;
use random;
use resources;
use InputState;

enum Status {
    Shooting,
    Walking,
    Idle,
}

pub struct StupidEnemy {
    obj: Rect,

    // action
    status: Status,
    ready: bool,
    time_until_ready: f32,

    // for flipping the image every now and then.
    movement_counter: f32,

    // movement
    current_velocity: (f32, f32),
    knockback_velocity: (f32, f32),
    speed: f32,

    // health related stuff
    hp: isize,
    has_rocketlauncher: bool,
}

impl StupidEnemy {
    pub fn new(pos: (f32, f32), level: u32) -> Self {
        let rocketlauncher_chance = (level / 2) * 20;
        let has_rocketlauncher = random::rand() % 101 <= rocketlauncher_chance;
        // Put some meaningful default values here.
        Self {
            obj: Rect {
                pos: pos,
                size: (16f32, 16f32),
            },

            status: Status::Idle,
            ready: false,
            time_until_ready: 1000f32,

            current_velocity: (0f32, 0f32),
            knockback_velocity: (0f32, 0f32),
            speed: 25f32 + level as f32,

            movement_counter: 0f32,

            hp: 2 + level as isize / 2,
            has_rocketlauncher,
        }
    }

    pub fn get_center_position(&self) -> AbsolutePosition {
        AbsolutePosition::from_f32((
            self.obj.pos.0 + self.obj.size.0 / 2f32,
            self.obj.pos.1 + self.obj.size.1 / 2f32,
        ))
    }
}

impl Collidable for StupidEnemy {
    fn hitbox(&self) -> Option<Rect> {
        let mut hitbox = self.obj.clone();
        hitbox.pos.0 += 3f32;
        hitbox.size.0 -= 6f32;
        Some(hitbox)
    }
}

impl Entity for StupidEnemy {
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
                    let target = level.player.borrow().position();
                    let vec = target.sub(center);
                    let vec = normalize((vec.0, vec.1));
                    // Scale normalized vector by the player graphics,
                    let vec = AbsolutePosition(vec.0, vec.1).mul(15f32);

                    if self.has_rocketlauncher {
                        &level.projectiles.borrow_mut().push(Box::new(
                            projectile::Rocket::new_from_absolute_coords(
                                &center.add(vec),
                                &target,
                                200f32,
                                projectile::Team::Villain,
                            ),
                        ));
                    } else {
                        &level.projectiles.borrow_mut().push(Box::new(
                            projectile::SimpleProjectile::new_from_absolute_coords(
                                &center.add(vec),
                                &target,
                                200f32,
                                1000f32,
                                projectile::Team::Villain,
                                color::ORANGE_YELLOW,
                            ),
                        ));
                    }

                    self.ready = true;
                }
            }
            Status::Walking => {
                if self.time_until_ready <= 0f32 {
                    self.ready = true;
                }
            }
        }
        if self.ready {
            if !inside_vision(
                &level.player.borrow().get_center_position(),
                &self.get_center_position(),
                &level.map.borrow(),
            ) {
                self.status = Status::Walking;
            } else {
                let randome_number = random::rand();
                match self.status {
                    Status::Idle => {
                        if randome_number % 10 > 2 {
                            self.status = Status::Shooting;
                        } else if randome_number % 10 > 5 {
                            self.status = Status::Idle;
                        } else {
                            self.status = Status::Walking;
                        }
                    }
                    Status::Shooting => {
                        if randome_number % 10 < 4 {
                            self.status = Status::Walking;
                        } else if randome_number % 10 < 8 {
                            self.status = Status::Shooting;
                        } else {
                            self.status = Status::Idle;
                        }
                    }
                    Status::Walking => {
                        if randome_number % 10 < 4 {
                            self.status = Status::Walking;
                        } else if randome_number % 10 < 8 {
                            self.status = Status::Shooting;
                        } else {
                            self.status = Status::Idle;
                        }
                    }
                }
            }
            self.time_until_ready = match self.status {
                Status::Idle => 500f32,
                Status::Shooting => 1000f32,
                Status::Walking => 1000f32,
            };
            self.ready = false;
        }

        match self.status {
            Status::Walking => {
                movement::move_to_player(
                    &self.hitbox().unwrap(),
                    &mut self.current_velocity,
                    self.speed,
                    level,
                );
            }
            _ => {}
        }

        movement::apply_force(
            &mut self.obj.pos,
            (16f32, 16f32),
            &mut self.current_velocity,
            &mut self.knockback_velocity,
            delta,
            level,
        );

        if math::len_vec2(self.current_velocity) > 0.5f32 {
            self.movement_counter += delta;
        }
    }

    fn draw(&self, canvas: &mut Canvas, camera: &Camera) {
        let tileset_data = gfx::RGBData::new(resources::TILESET, resources::TILESET_SIZE);;
        let tileset = gfx::Tileset::new(&tileset_data, (16, 16));

        let tile = tileset.get_tile((5, 0)).unwrap();
        let img = filter::TransparencyFilter::new(&tile, color::PINK);

        let flip = if (self.movement_counter / 500f32) as usize % 2 == 0 {
            filter::MirrorMode::None
        } else {
            filter::MirrorMode::Vertical
        };
        let img = filter::MirroringFilter::new(&img, flip);

        let sprite = gfx::Sprite::new(&img);

        sprite.draw(self.position().to_screen(camera), canvas);

        // draw weapon
        let tile = if self.has_rocketlauncher {
            tileset.get_tile((7, 0)).unwrap()
        } else {
            tileset.get_tile((6, 0)).unwrap()
        };
        let img = filter::TransparencyFilter::new(&tile, color::PINK);

        let flip = if self.current_velocity.0 > 0f32 {
            filter::MirrorMode::None
        } else {
            filter::MirrorMode::Vertical
        };
        let img = filter::MirroringFilter::new(&img, flip);

        let sprite = gfx::Sprite::new(&img);

        sprite.draw(self.position().to_screen(camera), canvas);
    }

    fn on_hit(&mut self, projectile: &projectile::Projectile, level: &level::Level) {
        self.hp -= projectile.damage();

        let vel = projectile.velocity();
        self.knockback_velocity.0 += vel.0 * projectile.onhit_recoil();
        self.knockback_velocity.1 += vel.1 * projectile.onhit_recoil();

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

impl Enemy for StupidEnemy {
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
