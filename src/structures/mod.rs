mod chest;
pub use self::chest::Chest;
mod barrel;
pub use self::barrel::Barrel;

use collision::Collidable;
use common::AbsolutePosition;
use entity::Entity;
use level;

pub trait Structure: Entity + Collidable {
    fn destroy(&mut self, &level::Level);
    fn is_alive(&self) -> bool;
    fn position(&self) -> AbsolutePosition;
}
