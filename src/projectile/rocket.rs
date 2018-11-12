use camera::Camera;
use collision::Collidable;
use color;
use common::AbsolutePosition;
use entity::Entity;
use gfx;
use gfx::Drawable;
use level;
use math;
use particle;
use projectile;
use random;
use InputState;

use alloc::boxed::Box;
use refcell::RefCell;

pub struct Rocket {
    pos: AbsolutePosition,
    velocity: (f32, f32),
    alive: bool,
    time_to_live: f32,
    team: projectile::Team,
    spawn_next_particles_at: f32,
}

impl Rocket {
    pub fn new_from_absolute_coords(
        src: &AbsolutePosition,
        dst: &AbsolutePosition,
        speed: f32,
        team: projectile::Team,
    ) -> Self {
        // Normalize velocity
        let vel_x = (dst.0 - src.0) as f32;
        let vel_y = (dst.1 - src.1) as f32;
        let (vel_x, vel_y) = math::normalize((vel_x, vel_y));

        Self {
            pos: *src,
            velocity: ((vel_x * speed), (vel_y * speed)),
            alive: true,
            // 2s
            time_to_live: 2000.0f32,
            team,
            spawn_next_particles_at: 0f32,
        }
    }
}

impl projectile::Projectile for Rocket {
    fn destroy(&mut self) {
        self.alive = false;
        self.time_to_live = -1f32;
    }

    fn is_alive(&self) -> bool {
        self.alive && self.pos.0 >= 0f32 && self.pos.1 >= 0f32
    }

    fn velocity(&self) -> (f32, f32) {
        self.velocity
    }

    fn position(&self) -> AbsolutePosition {
        self.pos
    }

    fn damage(&self) -> isize {
        3
    }

    fn onhit_recoil(&self) -> f32 {
        1.0f32
    }

    fn can_hit(&self, team: projectile::Team) -> bool {
        match (team, &self.team) {
            (projectile::Team::Player, projectile::Team::Player) => false,
            (projectile::Team::Villain, projectile::Team::Villain) => false,
            _ => true,
        }
    }

    fn particle_multiplier(&self) -> u32 {
        5
    }
}

impl Entity for Rocket {
    fn update(&mut self, _: &InputState, delta: f32, level: &level::Level) {
        self.pos.0 += self.velocity.0 * (delta / 1000f32);
        self.pos.1 += self.velocity.1 * (delta / 1000f32);
        if self.alive {
            self.time_to_live -= delta;
        }
        if self.time_to_live < 0f32 {
            self.alive = false;
        }

        if self.alive {
            if self.spawn_next_particles_at < delta {
                for _ in 0..15 {
                    // Different speed categories.
                    const TYPES: [(f32, f32); 5] = [
                        (0.1f32, 0.1f32),
                        (0.3f32, 0.2f32),
                        (0.5f32, 0.3f32),
                        (0.7f32, 0.4f32),
                        (1.0f32, 0.5f32),
                    ];

                    let t = TYPES[(random::rand() % 5) as usize];
                    let speed = t.0;
                    let lifetime = t.1;

                    // Random offset
                    let rx = ((random::rand() % 11 - 5) as f32) * 10f32 * speed;
                    let ry = ((random::rand() % 11 - 5) as f32) * 10f32 * speed;

                    level.particles.borrow_mut().push(RefCell::new(Box::new(
                        particle::DecayParticle::new(
                            self.pos.add(AbsolutePosition(4f32, 4f32)),
                            (-self.velocity.0 * speed + rx, -self.velocity.1 * speed + ry),
                            lifetime,
                            [
                                color::ORANGE_RED,
                                color::ORANGE,
                                color::ORANGE_YELLOW,
                                color::SUN_YELLOW,
                                color::BRIGHT_YELLOW,
                            ],
                            0.5f32,
                        ),
                    )));
                }
                self.spawn_next_particles_at = 10f32;
            } else {
                self.spawn_next_particles_at -= delta;
            }
        }
    }

    /// Draw the object to the canvas.
    fn draw(&self, canvas: &mut gfx::Canvas, camera: &Camera) {
        let color_func = |dist: f32| {
            if dist < 0.2f32 {
                Some(color::SILVER)
            } else if dist < 0.4f32 {
                Some(color::RED)
            } else if dist < 1.0f32 {
                Some(color::DARK_GRAY)
            } else {
                None
            }
        };
        let bullet_img_data = gfx::Circle::new(8, &color_func);
        let bullet_img = gfx::Sprite::new(&bullet_img_data);

        let pos = self.pos.to_screen(camera);
        bullet_img.draw(pos, canvas);
    }

    fn on_hit(&mut self, _: &projectile::Projectile, _: &level::Level) {
        // Well, we can't be hit so this is meaningless.
    }
}

impl Collidable for Rocket {
    fn hitbox(&self) -> Option<math::Rect> {
        Some(math::Rect {
            pos: (self.pos.0 + 2f32, self.pos.1 + 2f32),
            size: (4f32, 4f32),
        })
    }
}
