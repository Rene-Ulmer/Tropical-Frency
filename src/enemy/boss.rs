use alloc::boxed::Box;
use alloc::vec::Vec;
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
use misc::inside_vision;
use movement;
use powerup;
use projectile;
use random::rand;
use resources;
use weapon;
use weapon::Weapon;
use InputState;

enum Status {
    Shooting,
    Walking,
    Idle,
}

pub struct Boss {
    obj: math::Rect,

    // action
    status: Status,
    ready: bool,
    time_until_ready: f32,

    // movement
    current_velocity: (f32, f32),
    knockback_velocity: (f32, f32),
    speed: f32,

    // for flipping the image every now and then.
    movement_counter: f32,

    // shooty shooty stuff
    weapons: Vec<Box<weapon::Weapon>>,
    selected_weapon: usize,

    // health related stuff
    hp: isize,
}

impl Boss {
    pub fn new(pos: (f32, f32), level: u32) -> Self {
        // Put some meaningful default values here.

        let mut weapons: Vec<Box<weapon::Weapon>> = Vec::new();
        let mut rocketlauncher = weapon::Rocketlauncher::new();
        for _ in 0..level / 2 {
            rocketlauncher.on_powerup_pickup();
        }
        weapons.push(Box::new(rocketlauncher));
        weapons.push(Box::new(weapon::Pistol::new()));

        Self {
            obj: math::Rect {
                pos: pos,
                size: (32f32, 32f32),
            },

            status: Status::Walking,
            ready: false,
            time_until_ready: 1000f32,

            current_velocity: (0f32, 0f32),
            knockback_velocity: (0f32, 0f32),
            speed: 25f32 + level as f32 / 2f32,

            movement_counter: 0f32,

            weapons: weapons,
            selected_weapon: 0,

            hp: 10 + level as isize * 2,
        }
    }

    pub fn get_center_position(&self) -> AbsolutePosition {
        AbsolutePosition::from_f32((
            self.obj.pos.0 + self.obj.size.0 / 2f32,
            self.obj.pos.1 + self.obj.size.1 / 2f32,
        ))
    }

    pub fn switch_weapon(&mut self) {
        self.selected_weapon = (self.selected_weapon + 1) % self.weapons.len();
    }
}

impl Collidable for Boss {
    fn hitbox(&self) -> Option<math::Rect> {
        let mut hitbox = self.obj.clone();
        // Times 2 because of the scaling of the enemy.
        hitbox.pos.0 += 3f32 * 2f32;
        hitbox.size.0 -= 6f32 * 2f32;
        Some(hitbox)
    }
}

impl Entity for Boss {
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
                    let vec = math::normalize((vec.0, vec.1));
                    // Scale normalized vector by the player graphics,
                    let vec = AbsolutePosition(vec.0, vec.1).mul(25f32);

                    self.weapons[self.selected_weapon]
                        .shoot(&center.add(vec), &target, level, projectile::Team::Villain)
                        .ok()
                        .is_some();
                    if self.weapons[self.selected_weapon].ammo_in_clip() == 0 {
                        self.weapons[self.selected_weapon].reload();
                        self.switch_weapon();
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
                let randome_number = rand();
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
                Status::Shooting => 500f32,
                Status::Walking => 1000f32,
            };
            self.ready = false;
        }

        match self.status {
            Status::Walking => {}
            _ => {
                return;
            }
        }

        movement::move_to_player(
            &self.hitbox().unwrap(),
            &mut self.current_velocity,
            self.speed,
            level,
        );
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

        // draw body
        let tile = tileset.get_tile((5, 0)).unwrap();
        let img_transparent = filter::TransparencyFilter::new(&tile, color::PINK);
        let img = filter::ScalingFilter::new(&img_transparent, 2f32);

        // flip depending on movement_counter
        let flip = if (self.movement_counter / 500f32) as usize % 2 == 0 {
            filter::MirrorMode::None
        } else {
            filter::MirrorMode::Vertical
        };
        let img = filter::MirroringFilter::new(&img, flip);

        let sprite = gfx::Sprite::new(&img);

        sprite.draw(self.position().to_screen(camera), canvas);

        // draw weapon
        // img depending on selected_weapon
        let tile = if self.selected_weapon == 0 {
            tileset.get_tile((7, 0)).unwrap()
        } else {
            tileset.get_tile((6, 0)).unwrap()
        };
        let img_transparent = filter::TransparencyFilter::new(&tile, color::PINK);
        let img = filter::ScalingFilter::new(&img_transparent, 2f32);

        // flip depending on current_velocity
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
        self.knockback_velocity.0 += vel.0 * (projectile.onhit_recoil() / 2f32);
        self.knockback_velocity.1 += vel.1 * (projectile.onhit_recoil() / 2f32);

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

impl Enemy for Boss {
    fn die(&mut self, level: &level::Level) {
        powerup::maybe_drop_powerup(self.position(), 100, level);
    }

    fn is_alive(&self) -> bool {
        self.hp > 0
    }

    fn position(&self) -> AbsolutePosition {
        AbsolutePosition::from_f32(self.obj.pos)
    }
}
