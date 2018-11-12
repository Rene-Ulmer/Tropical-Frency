use camera::Camera;
use collision::Collidable;
use common::AbsolutePosition;
use entity::Entity;
use gfx::Canvas;
use level;
use math::Rect;
use particle::Particle;
use projectile::Projectile;
use InputState;

pub struct DecayParticle {
    pos: AbsolutePosition,
    velocity: (f32, f32),
    max_time_to_live: f32,
    time_to_live: f32,

    colors: [u32; 5],
    slowdown: f32,
}

impl DecayParticle {
    pub fn new(
        pos: AbsolutePosition,
        velocity: (f32, f32),
        time_to_live: f32,
        colors: [u32; 5],
        slowdown: f32,
    ) -> Self {
        Self {
            pos,
            velocity,
            time_to_live,
            max_time_to_live: time_to_live,
            colors,
            slowdown,
        }
    }
}

impl Collidable for DecayParticle {
    fn hitbox(&self) -> Option<Rect> {
        None
    }
}

impl Entity for DecayParticle {
    fn update(&mut self, _: &InputState, delta: f32, _: &level::Level) {
        self.pos.0 += self.velocity.0 * (delta / 1000f32);
        self.pos.1 += self.velocity.1 * (delta / 1000f32);
        self.velocity.0 *= 1f32 - self.slowdown;
        self.velocity.1 *= 1f32 - self.slowdown;
        self.time_to_live -= delta / 1000f32;
    }

    fn draw(&self, canvas: &mut Canvas, camera: &Camera) {
        // Calculate percent of decay.
        let d = (4.99999f32 * self.time_to_live / self.max_time_to_live) as usize;

        let p = self.pos.to_screen(camera);
        canvas
            .set_pixel(p.0 as usize, p.1 as usize, self.colors[d])
            .ok();
    }

    fn on_hit(&mut self, _: &Projectile, _: &level::Level) {}
}

impl Particle for DecayParticle {
    fn is_alive(&self) -> bool {
        self.time_to_live > 0f32
    }
}
