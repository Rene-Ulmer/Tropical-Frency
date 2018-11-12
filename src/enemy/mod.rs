mod stupidenemy;
pub use self::stupidenemy::StupidEnemy;
mod tower;
pub use self::tower::Tower;
mod runningbomb;
pub use self::runningbomb::RunningBomb;
mod boss;
pub use self::boss::Boss;

use collision::Collidable;
use common;
use entity::Entity;
use level;

pub trait Enemy: Entity + Collidable {
    fn die(&mut self, &level::Level);
    fn is_alive(&self) -> bool;
    fn position(&self) -> common::AbsolutePosition;
}
