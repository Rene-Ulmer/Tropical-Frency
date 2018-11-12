mod simpleparticle;
pub use self::simpleparticle::SimpleParticle;
use common::AbsolutePosition;

mod decayparticle;
pub use self::decayparticle::DecayParticle;

use level;
use math;
use particle;
use random;

use alloc::boxed::Box;
use refcell::RefCell;

pub trait Particle: ::entity::Entity + ::collision::Collidable {
    fn is_alive(&self) -> bool;
}

pub fn spawn_radial_particles(
    level: &level::Level,
    center: AbsolutePosition,
    color: u32,
    count: usize,
    min_speed: u32,
    max_speed: u32,
) {
    if max_speed <= min_speed {
        panic!("...");
    }
    let particles = level.particles.borrow_mut();
    for _ in 0..count {
        let angle = (random::rand() % (360 * 100)) as f32 / 100f32;
        let angle = math::deg_to_rad(angle);
        let speed =
            (random::rand() % (10 * (max_speed - min_speed))) as f32 / 10f32 + min_speed as f32;
        let vx = math::sin(angle) * speed;
        let vy = math::cos(angle) * speed;
        particles.push(RefCell::new(Box::new(particle::SimpleParticle::new(
            center,
            (vx, vy),
            0.5f32 + (random::rand() % 10) as f32 / 10f32,
            color,
        ))));
    }
}

pub fn spawn_directional_particles(
    level: &level::Level,
    center: AbsolutePosition,
    width: f32,
    direction: f32,
    color: u32,
    count: usize,
) {
    // Make sure width is positive and not eqal to zero.
    let width = if width < 1.0f32 { 1.0f32 } else { width };
    let offset = direction - width / 2f32;
    let particles = level.particles.borrow_mut();
    for _ in 0..count {
        let angle = (random::rand() % (width as u32 * 100)) as f32 / 100f32;
        let angle = math::deg_to_rad(angle + offset);
        let speed = (random::rand() % 1000) as f32 * 5f32 / 100f32 + 25f32;
        let vx = math::sin(angle) * speed;
        let vy = math::cos(angle) * speed;

        particles.push(RefCell::new(Box::new(particle::SimpleParticle::new(
            center,
            (vx, vy),
            0.5f32 + (random::rand() % 10) as f32 / 10f32,
            color,
        ))));
    }
}
