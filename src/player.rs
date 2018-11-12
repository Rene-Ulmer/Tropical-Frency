use alloc::boxed::Box;
use alloc::vec::Vec;
use camera::Camera;
use collision;
use collision::Collidable;
use color;
use common;
use effects;
use entity::Entity;
use filter;
use gfx;
use gfx::Drawable;
use level;
use map;
use math;
use misc;
use movement;
use projectile;
use resources;
use weapon;
use InputState;
use HEIGHT;

// helper method to make sure we always use the same values for the hitbox
// calculation.
fn hitbox_for_pos(pos: (f32, f32)) -> math::Rect {
    math::Rect {
        pos: (pos.0 + 4f32, pos.1),
        size: (8f32, 16f32),
    }
}

// Basically an inverse of the above
fn hitbox_to_pos(hitbox: &math::Rect) -> (f32, f32) {
    (hitbox.pos.0 - 4f32, hitbox.pos.1)
}

pub enum WeaponState {
    Ready,
    Reloading(f32),
}

pub enum PlayerState {
    Regular,
    BeamingOut(f32, usize, common::AbsolutePosition),
    BeamingIn(f32, usize),
}

pub struct Player {
    obj: math::Rect,

    // movement
    max_speed: f32,
    acceleration: f32,
    deceleration: f32,
    current_velocity: (f32, f32),
    knockback_velocity: (f32, f32),

    // health related stuff
    pub hp: isize,
    invuln_time: f32,
    current_invuln_time: f32,

    // shooty shooty stuff
    pub weapons: Vec<Box<weapon::Weapon>>,
    selected_weapon: usize,
    weapon_state: WeaponState,

    player_state: PlayerState,
    mirror_player: bool,
}

impl Player {
    pub fn new() -> Self {
        let mut weapons: Vec<Box<weapon::Weapon>> = Vec::new();
        weapons.push(Box::new(weapon::Pistol::new()));
        weapons.push(Box::new(weapon::Rocketlauncher::new()));

        Self {
            obj: math::Rect {
                pos: (32f32, 16f32),
                size: (16f32, 16f32),
            },

            max_speed: 50f32,
            acceleration: 100f32,
            deceleration: 75f32,
            current_velocity: (0f32, 0f32),
            knockback_velocity: (0f32, 0f32),

            hp: 5,
            invuln_time: 1000f32,
            current_invuln_time: 0.0f32,

            weapons: weapons,
            selected_weapon: 0,
            weapon_state: WeaponState::Ready,

            player_state: PlayerState::Regular,

            mirror_player: false,
        }
    }

    pub fn update_facing(&mut self, mouse: &common::AbsolutePosition) {
        self.mirror_player = mouse.0 < hitbox_for_pos(self.obj.pos).pos.0;
    }

    pub fn weapon_mut(&mut self) -> &mut Box<weapon::Weapon> {
        &mut self.weapons[self.selected_weapon]
    }

    pub fn weapon(&self) -> &Box<weapon::Weapon> {
        &self.weapons[self.selected_weapon]
    }

    pub fn dodge_beam(&mut self, map: &map::Map) {
        if self.hp <= 0 {
            return;
        }
        // Can only beam to cursor when not already beaming
        match self.player_state {
            PlayerState::Regular => {}
            _ => {
                return;
            }
        }

        // Only blink if the player is actually moving.
        if math::len_vec2(self.current_velocity) < 0.01f32 {
            return;
        }
        let target = math::add_vec2(self.get_center_position().to_tuple(), self.current_velocity);

        // The target should represent the center of the player.
        let target = math::sub_vec2(target, (8f32, 8f32));

        // Player can only blink a short distance, so scale it down if it
        // exceeds the maximum beam distance.
        let path = math::sub_vec2(target, self.obj.pos);

        // Let's beam as far as we're able to. For this we just do some brute
        // force.
        let mut path_len = 100f32;
        while path_len > 10f32 {
            let path_scaled = math::scale_vec2(path, path_len);
            let path_scaled = math::add_vec2(path_scaled, self.obj.pos);
            let target = common::AbsolutePosition(path_scaled.0, path_scaled.1);

            // Perform collision detection at target position.
            let target_hitbox = hitbox_for_pos(target.to_tuple());

            if misc::inside_vision(&self.get_center_position(), &target, &map)
                && !collision::collides_with_map(&target_hitbox, &map)
            {
                self.beam_to(target);
                return;
            }

            path_len -= 10f32;
        }
    }

    pub fn beam_to(&mut self, pos: common::AbsolutePosition) {
        self.player_state = PlayerState::BeamingOut(0f32, 2, pos);
    }

    pub fn switch_weapon(&mut self) {
        if self.hp <= 0 {
            return;
        }
        match self.player_state {
            PlayerState::Regular => {}
            _ => {
                return;
            }
        }

        // Cancel reload if in progress.
        self.weapon_state = WeaponState::Ready;
        self.selected_weapon = (self.selected_weapon + 1) % self.weapons.len();
    }

    pub fn position(&self) -> common::AbsolutePosition {
        common::AbsolutePosition::from_f32(self.obj.pos)
    }

    pub fn get_center_position(&self) -> common::AbsolutePosition {
        common::AbsolutePosition::from_f32((
            self.obj.pos.0 + self.obj.size.0 / 2f32,
            self.obj.pos.1 + self.obj.size.1 / 2f32,
        ))
    }

    pub fn shoot(
        &mut self,
        pos: &common::AbsolutePosition,
        aim: &common::AbsolutePosition,
        level: &level::Level,
    ) -> bool {
        if self.hp <= 0 {
            return false;
        }
        match self.player_state {
            PlayerState::Regular => {}
            _ => {
                return false;
            }
        }
        match self.weapon_state {
            WeaponState::Reloading(_) => false,
            WeaponState::Ready => self
                .weapon_mut()
                .shoot(pos, aim, level, projectile::Team::Player)
                .ok()
                .is_some(),
        }
    }

    pub fn reload(&mut self) {
        if self.hp <= 0 {
            return;
        }
        match self.player_state {
            PlayerState::Regular => {}
            _ => {
                return;
            }
        }

        match self.weapon_state {
            WeaponState::Ready => {
                if self.weapon().ammo_in_clip() < self.weapon().max_clip_ammo() {
                    self.weapon_state = WeaponState::Reloading(self.weapon().reload_time());
                }
            }
            _ => {}
        }
    }
}

impl Collidable for Player {
    fn hitbox(&self) -> Option<math::Rect> {
        match self.player_state {
            PlayerState::Regular => Some(hitbox_for_pos(self.obj.pos)),
            _ => None,
        }
    }
}

impl Entity for Player {
    fn update(&mut self, input: &InputState, delta: f32, level: &level::Level) {
        // Handle invuln times.
        if self.current_invuln_time > 0f32 {
            self.current_invuln_time -= delta;
        }

        self.player_state = match self.player_state {
            PlayerState::Regular => PlayerState::Regular,
            PlayerState::BeamingOut(mut time, mut n, destination) => {
                time += delta;
                if time > 50f32 {
                    n += 1;
                    time = 0f32;
                }
                if n > 8 {
                    self.obj.pos = destination.to_tuple();
                    PlayerState::BeamingIn(0f32, 2)
                } else {
                    PlayerState::BeamingOut(time, n, destination)
                }
            }
            PlayerState::BeamingIn(mut time, mut n) => {
                time += delta;
                if time > 50f32 {
                    n += 1;
                    time = 0f32;
                }
                if n > 8 {
                    PlayerState::Regular
                } else {
                    PlayerState::BeamingIn(time, n)
                }
            }
        };

        match self.weapon_state {
            WeaponState::Reloading(n) => {
                if delta / 1000f32 >= n {
                    self.weapon_mut().reload();
                    self.weapon_state = WeaponState::Ready;
                } else {
                    self.weapon_state = WeaponState::Reloading(n - delta / 1000f32);
                }
            }
            _ => {}
        }

        // Apply forces only when not beaming.
        match self.player_state {
            PlayerState::Regular => {}
            _ => {
                return;
            }
        }
        if self.hp <= 0 {
            return;
        }

        // wasd as input keys.
        const KEY_LEFT: usize = 65;
        const KEY_RIGHT: usize = 68;
        const KEY_UP: usize = 87;
        const KEY_DOWN: usize = 83;

        let accel = self.acceleration * delta / 1000f32;
        let decel = self.deceleration * delta / 1000f32;

        if input.key_down[KEY_LEFT] {
            self.current_velocity.0 -= accel;
        } else if input.key_down[KEY_RIGHT] {
            self.current_velocity.0 += accel;
        } else {
            if self.current_velocity.0 > decel {
                self.current_velocity.0 -= decel;
            } else if self.current_velocity.0 > 0f32 {
                self.current_velocity.0 = 0f32;
            } else if self.current_velocity.0 < -decel {
                self.current_velocity.0 += decel;
            } else if self.current_velocity.0 < 0f32 {
                self.current_velocity.0 = 0f32;
            }
        }

        self.current_velocity.0 =
            common::apply_bounds(self.current_velocity.0, -self.max_speed, self.max_speed);

        if input.key_down[KEY_UP] {
            self.current_velocity.1 -= accel;
        } else if input.key_down[KEY_DOWN] {
            self.current_velocity.1 += accel;
        } else {
            if self.current_velocity.1 > decel {
                self.current_velocity.1 -= decel;
            } else if self.current_velocity.1 > 0f32 {
                self.current_velocity.1 = 0f32;
            } else if self.current_velocity.1 < -decel {
                self.current_velocity.1 += decel;
            } else if self.current_velocity.1 < 0f32 {
                self.current_velocity.1 = 0f32;
            }
        }

        self.current_velocity.1 =
            common::apply_bounds(self.current_velocity.1, -self.max_speed, self.max_speed);

        let mut hitbox = hitbox_for_pos(self.obj.pos);
        movement::apply_force(
            &mut hitbox.pos,
            hitbox.size.clone(),
            &mut self.current_velocity,
            &mut self.knockback_velocity,
            delta,
            level,
        );

        self.obj.pos = hitbox_to_pos(&hitbox);
    }

    fn draw(&self, canvas: &mut gfx::Canvas, camera: &Camera) {
        let ap = common::AbsolutePosition(self.obj.pos.0, self.obj.pos.1).round();
        let p = ap.to_screen(camera);

        let tileset_data = gfx::RGBData::new(resources::TILESET, resources::TILESET_SIZE);;
        let tileset = gfx::Tileset::new(&tileset_data, (16, 16));

        // Draw player
        let player_tile = tileset.get_tile((4, 0)).unwrap();
        let player_img = filter::TransparencyFilter::new(&player_tile, color::PINK);

        let truncate_x = match self.player_state {
            PlayerState::Regular => 0,
            PlayerState::BeamingOut(_, n, _) => n,
            PlayerState::BeamingIn(_, n) => 8 - n,
        };

        let player_img = filter::TruncatingFilter::new(&player_img, truncate_x, 0);
        let player_img = filter::MirroringFilter::new(
            &player_img,
            if self.hp <= 0 {
                filter::MirrorMode::Horizontal
            } else if self.mirror_player {
                filter::MirrorMode::Vertical
            } else {
                filter::MirrorMode::None
            },
        );

        let draw_red = (self.current_invuln_time / 100f32) as usize % 2 == 1;
        if draw_red {
            let player_img = filter::MonoFilter::new(&player_img, color::RED);
            let player_sprite = gfx::Sprite::new(&player_img);
            player_sprite.draw(p, canvas);
        } else {
            let player_sprite = gfx::Sprite::new(&player_img);
            player_sprite.draw(p, canvas);
        };

        // Add reloading bar.
        match self.weapon_state {
            WeaponState::Reloading(time_left) => {
                // Let's draw some reloading stuff.
                let percent_completed = time_left / self.weapon().reload_time();
                gfx::draw_line(
                    canvas,
                    ap.add(common::AbsolutePosition(0f32, 18f32))
                        .to_screen(camera),
                    ap.add(common::AbsolutePosition(percent_completed * 16f32, 18f32))
                        .to_screen(camera),
                    color::DARK_GRAY,
                );
            }
            _ => {}
        }

        // draw ammo count
        let ammo_tile = match self.selected_weapon {
            0 => tileset.get_tile((1, 1)).unwrap(),
            1 => tileset.get_tile((3, 1)).unwrap(),
            _ => unreachable!(),
        };
        let ammo_img = filter::TransparencyFilter::new(&ammo_tile, 0);
        let ammo_gray_img = filter::MonoFilter::new(&ammo_img, color::DARK_GRAY);
        let ammo_sprite = gfx::Sprite::new(&ammo_img);
        let ammo_gray_sprite = gfx::Sprite::new(&ammo_gray_img);

        for x in 0..self.weapon().max_clip_ammo() {
            let ammo_draw_pos = common::ScreenPosition {
                0: 10 + 8 * x as isize,
                1: HEIGHT as isize - 30,
            };
            if x < self.weapon().ammo_in_clip() {
                ammo_sprite.draw(ammo_draw_pos, canvas);
            } else {
                ammo_gray_sprite.draw(ammo_draw_pos, canvas);
            }
        }

        // draw player health
        let health_tile = tileset.get_tile((0, 1)).unwrap();
        let health_img = filter::TransparencyFilter::new(&health_tile, 0);
        let health_sprite = gfx::Sprite::new(&health_img);

        for y in 0..=1 {
            for x in 0..10 {
                if self.hp <= y * 10 + x {
                    break;
                }
                let health_draw_pos = common::ScreenPosition {
                    0: 10 + x * 8,
                    1: HEIGHT as isize - 50 + y * 8,
                };
                health_sprite.draw(health_draw_pos, canvas);
            }
        }
    }

    fn on_hit(&mut self, projectile: &projectile::Projectile, level: &level::Level) {
        if self.hp <= 0 {
            return;
        }
        if self.current_invuln_time <= 0f32 {
            self.hp -= projectile.damage();

            let vel = projectile.velocity();
            self.knockback_velocity.0 += vel.0 * projectile.onhit_recoil();
            self.knockback_velocity.1 += vel.1 * projectile.onhit_recoil();
            self.current_invuln_time = self.invuln_time;

            effects::on_hit_effect(
                projectile,
                level,
                self.get_center_position(),
                1,
                color::RED,
                color::DARK_RED,
            );
        }
    }
}
