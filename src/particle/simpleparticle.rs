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

pub struct SimpleParticle {
    pos: AbsolutePosition,
    velocity: (f32, f32),
    time_to_live: f32,

    // Maybe add some color degrading stuff?
    // Maybe make this a trait instead and implement various particles.
    color: u32,
}

impl SimpleParticle {
    pub fn new(pos: AbsolutePosition, velocity: (f32, f32), time_to_live: f32, color: u32) -> Self {
        Self {
            pos,
            velocity,
            time_to_live,
            color,
        }
    }
}

impl Collidable for SimpleParticle {
    fn hitbox(&self) -> Option<Rect> {
        None
    }
}

impl Entity for SimpleParticle {
    fn update(&mut self, _: &InputState, delta: f32, _: &level::Level) {
        self.pos.0 += self.velocity.0 * (delta / 1000f32);
        self.pos.1 += self.velocity.1 * (delta / 1000f32);
        self.time_to_live -= delta / 1000f32;
    }

    fn draw(&self, canvas: &mut Canvas, camera: &Camera) {
        let p = self.pos.to_screen(camera);
        canvas
            .set_pixel(p.0 as usize, p.1 as usize, self.color)
            .ok();
    }

    fn on_hit(&mut self, _: &Projectile, _: &level::Level) {}
}

impl Particle for SimpleParticle {
    fn is_alive(&self) -> bool {
        self.time_to_live > 0f32
    }
}
