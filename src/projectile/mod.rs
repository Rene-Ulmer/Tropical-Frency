mod rocket;
pub use self::rocket::Rocket;

mod simpleprojectile;
pub use self::simpleprojectile::SimpleProjectile;

use collision::Collidable;
use common::AbsolutePosition;
use entity::Entity;

pub enum Team {
    Player,
    Villain,
    Neutral,
}

pub trait Projectile: Entity + Collidable {
    /// Called when the bullet hit the target (and should be destroyed).
    fn destroy(&mut self);
    fn is_alive(&self) -> bool;
    fn velocity(&self) -> (f32, f32);
    fn position(&self) -> AbsolutePosition;
    fn damage(&self) -> isize;
    fn onhit_recoil(&self) -> f32;

    fn can_hit(&self, Team) -> bool;
    fn particle_multiplier(&self) -> u32;
}
