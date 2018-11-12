use alloc::boxed::Box;
use camera;
use collision;
use common;
use entity;
use gfx;
use level;
use math;
use particle;
use projectile;
use InputState;

pub fn on_hit_effect(
    projectile: &projectile::Projectile,
    level: &level::Level,
    center: common::AbsolutePosition,
    multiplier: u32,
    color1: u32,
    color2: u32,
) {
    let vel = projectile.velocity();
    let angle = math::rad_to_deg(math::atan2(-vel.1, vel.0)) + 90f32;

    let multiplier = multiplier * projectile.particle_multiplier();

    particle::spawn_radial_particles(
        level,
        center,
        color1,
        20 * multiplier as usize, // count
        3,                        // min_speed
        10 * multiplier,          // max_speed
    );

    particle::spawn_directional_particles(
        level,
        center,
        30f32,
        angle,
        color2,
        5 * multiplier as usize,
    );
}

pub struct BulletSpawner {
    position: common::AbsolutePosition,
    time: f32,
    pulses: usize,
    bullets_per_pulse: usize,
    pulse_count: usize,
    time_between_pulses: f32,

    bullet_spawn_fn: Box<Fn(&level::Level, &common::AbsolutePosition, &common::AbsolutePosition)>, // From -> To.
}

impl BulletSpawner {
    pub fn new<T>(
        position: &common::AbsolutePosition,
        bullets_per_pulse: usize,
        pulse_count: usize,
        time_between_pulses: f32,
        bullet_spawn_fn: T,
    ) -> Self
    where
        T: Fn(&level::Level, &common::AbsolutePosition, &common::AbsolutePosition) + 'static,
    {
        Self {
            position: *position,
            time: 0f32,
            pulses: 0,
            bullets_per_pulse,
            pulse_count,
            time_between_pulses,
            bullet_spawn_fn: Box::new(bullet_spawn_fn),
        }
    }

    pub fn is_alive(&self) -> bool {
        self.time <= self.pulse_count as f32 * self.time_between_pulses
    }
}

impl collision::Collidable for BulletSpawner {
    fn hitbox(&self) -> Option<math::Rect> {
        None
    }
}

impl entity::Entity for BulletSpawner {
    fn update(&mut self, _input_state: &InputState, delta: f32, level: &level::Level) {
        self.time += delta;

        if ((self.time / self.time_between_pulses) as usize + 1) > self.pulses {
            let deg_per_i = 360f32 / self.bullets_per_pulse as f32;
            let diff = (self.pulses as f32 / self.pulse_count as f32) * 360f32;

            for i in 0..self.bullets_per_pulse {
                let deg = deg_per_i * i as f32 + diff;
                let x = math::sin(math::deg_to_rad(deg));
                let y = math::cos(math::deg_to_rad(deg));

                let direction = common::AbsolutePosition(x, y);

                /*
                level.projectiles.borrow_mut().push(Box::new(
                    projectile::SimpleProjectile::new_from_absolute_coords(
                        &self.position,
                        &self.position.add(direction),
                        50f32,
                        projectile::Team::Neutral,
                    ),
                ));
                */
                (self.bullet_spawn_fn)(level, &self.position, &self.position.add(direction));
            }

            self.pulses += 1;
        }
    }

    fn draw(&self, _canvas: &mut gfx::Canvas, _camera: &camera::Camera) {}
    fn on_hit(&mut self, _projectile: &projectile::Projectile, _level: &level::Level) {}
}
