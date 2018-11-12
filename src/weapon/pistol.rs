use alloc::boxed::Box;
use color;
use common::AbsolutePosition;
use level;
use projectile;
use weapon::Weapon;

pub struct Pistol {
    current_ammo: usize,
    max_clip: usize,
}

impl Pistol {
    pub fn new() -> Self {
        Self {
            current_ammo: 5,
            max_clip: 5,
        }
    }
}

impl Weapon for Pistol {
    fn shoot(
        &mut self,
        pos: &AbsolutePosition,
        aim: &AbsolutePosition,
        level: &level::Level,
        team: projectile::Team,
    ) -> Result<(), ()> {
        if self.current_ammo > 0 {
            level.projectiles.borrow_mut().push(Box::new(
                projectile::SimpleProjectile::new_from_absolute_coords(
                    pos,
                    aim,
                    400f32,
                    2000f32,
                    team,
                    color::ORANGE_YELLOW,
                ),
            ));
            self.current_ammo -= 1;
            Ok(())
        } else {
            Err(())
        }
    }

    fn reload_time(&self) -> f32 {
        0.5f32
    }

    /// Marks the weapon as reloaded.
    fn reload(&mut self) {
        self.current_ammo = self.max_clip_ammo();
    }

    /// Max ammo this weapon can have (in it's clip).
    fn max_clip_ammo(&self) -> usize {
        self.max_clip
    }

    fn ammo_in_clip(&self) -> usize {
        self.current_ammo
    }

    // Pistol has always ammo.
    fn total_ammo(&self) -> usize {
        9001
    }

    fn on_powerup_pickup(&mut self) {
        if self.max_clip < 14 {
            self.max_clip += 1;
        } else {
            self.current_ammo = self.max_clip;
        }
    }
}
