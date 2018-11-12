use common::AbsolutePosition;
use level;
use projectile;

mod pistol;
pub use self::pistol::Pistol;

mod rocketlauncher;
pub use self::rocketlauncher::Rocketlauncher;

pub trait Weapon {
    /// Shoots from pos using dir vector aim.
    fn shoot(
        &mut self,
        pos: &AbsolutePosition,
        aim: &AbsolutePosition,
        level: &level::Level,
        team: projectile::Team,
    ) -> Result<(), ()>;

    /// Returns the reload time in seconds.
    fn reload_time(&self) -> f32;

    /// Marks the weapon as reloaded.
    fn reload(&mut self);

    /// Max ammo this weapon can have (in it's clip).
    fn max_clip_ammo(&self) -> usize;

    /// Current ammo in the clip.
    fn ammo_in_clip(&self) -> usize;

    /// Total ammo (outside of clip).
    fn total_ammo(&self) -> usize;

    // Maybe some player slowdown factor?

    fn on_powerup_pickup(&mut self);
}
