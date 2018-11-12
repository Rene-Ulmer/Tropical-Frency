use common::AbsolutePosition;
use level;
use projectile;
use weapon::Weapon;

use alloc::boxed::Box;

pub struct Rocketlauncher {
    current_ammo: usize,
    max_clip: usize,
}

impl Rocketlauncher {
    pub fn new() -> Self {
        Self {
            current_ammo: 1,
            max_clip: 1,
        }
    }
}

impl Weapon for Rocketlauncher {
    fn shoot(
        &mut self,
        pos: &AbsolutePosition,
        aim: &AbsolutePosition,
        level: &level::Level,
        team: projectile::Team,
    ) -> Result<(), ()> {
        if self.current_ammo > 0 {
            level.projectiles.borrow_mut().push(Box::new(
                projectile::Rocket::new_from_absolute_coords(pos, aim, 300f32, team),
            ));
            self.current_ammo -= 1;
            Ok(())
        } else {
            Err(())
        }
    }

    fn reload_time(&self) -> f32 {
        2f32
    }

    /// Marks the weapon as reloaded.
    fn reload(&mut self) {
        if self.total_ammo() > self.max_clip_ammo() {
            self.current_ammo = self.max_clip_ammo();
        } else {
            self.current_ammo = self.total_ammo();
        }
    }

    /// Max ammo this weapon can have (in it's clip).
    fn max_clip_ammo(&self) -> usize {
        self.max_clip
    }

    fn ammo_in_clip(&self) -> usize {
        self.current_ammo
    }

    fn total_ammo(&self) -> usize {
        5
    }

    fn on_powerup_pickup(&mut self) {
        if self.max_clip < 5 {
            self.max_clip += 1;
        } else {
            self.current_ammo = self.max_clip;
        }
    }
}
