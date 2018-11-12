use alloc::boxed::Box;
use alloc::vec::Vec;
use camera;
use collision;
use collision::Collidable;
use common::AbsolutePosition;
use effects;
use enemy::{Boss, Enemy, RunningBomb, StupidEnemy, Tower};
use entity::Entity;
use game::Game;
use gfx::Canvas;
use input;
use map;
use math;
use particle;
use player;
use powerup;
use projectile;
use random;
use refcell::RefCell;
use trigger::Trigger;

use structures::{Barrel, Chest, Structure};
use InputState;

pub struct Level {
    pub enemies: Vec<RefCell<Box<Enemy>>>,
    pub map: RefCell<map::Map>,
    pub particles: RefCell<Vec<RefCell<Box<particle::Particle>>>>,
    pub player: RefCell<player::Player>,
    pub projectiles: RefCell<Vec<Box<projectile::Projectile>>>,
    pub structures: Vec<RefCell<Box<Structure>>>,
    pub trigger: Vec<RefCell<Trigger>>,

    pub misc: RefCell<Vec<effects::BulletSpawner>>,
    pub powerup: RefCell<Vec<powerup::Powerup>>,
}

impl Level {
    fn get_random_free_position(&self, size: (f32, f32), honor_nospawn: bool) -> AbsolutePosition {
        const MAX_X: usize = map::TILE_WIDTH as usize * map::MAP_SIZE;
        const MAX_Y: usize = map::TILE_HEIGHT as usize * map::MAP_SIZE;
        'outter: loop {
            // 1) Generate random coordinate inside the map.
            let x = random::rand() % (MAX_X as u32 - size.0 as u32);
            let y = random::rand() % (MAX_Y as u32 - size.1 as u32);

            if honor_nospawn && Level::is_in_nospawn_zone(AbsolutePosition(x as f32, y as f32)) {
                continue;
            }

            let hitbox = math::Rect {
                pos: (x as f32, y as f32),
                size,
            };

            // 2) Test whether the position is blocked by something.
            // Map
            if collision::hitbox_collides_with_map(hitbox, &self.map.borrow()) {
                continue;
            }

            // Object
            for obj in &self.structures {
                if let Some(ref ohitbox) = obj.borrow().hitbox() {
                    if hitbox.overlaps(ohitbox) {
                        continue 'outter;
                    }
                }
            }

            // Player
            if let Some(ref phitbox) = self.player.borrow().hitbox() {
                if hitbox.overlaps(phitbox) {
                    continue 'outter;
                }
            }

            // Enemy
            for enemy in &self.enemies {
                if let Some(ref ehitbox) = enemy.borrow().hitbox() {
                    if hitbox.overlaps(ehitbox) {
                        continue 'outter;
                    }
                }
            }

            return AbsolutePosition(x as f32, y as f32);
        }
    }

    fn is_in_nospawn_zone(pos: AbsolutePosition) -> bool {
        pos.0 < 10f32 * 16f32 && pos.1 < 10f32 * 16f32
    }

    pub fn generate_level(&mut self, level: u32) {
        self.map = RefCell::new(map::Map::new());

        // Replace the current level with a new one.
        // Drop all bullet spawners, projectiles, powerups, structures etc.
        self.misc.borrow_mut().clear();
        self.projectiles.borrow_mut().clear();
        self.powerup.borrow_mut().clear();
        self.structures.clear();

        // We make it more difficult by using barrels instead of chests.
        let n_structures = 10;

        for _ in 0..n_structures {
            // Generate a random structure.
            let pos = self.get_random_free_position((16f32, 16f32), false);
            if level < 5 || random::rand() % ((level - 3) / 2) == 0 {
                self.structures
                    .push(RefCell::new(Box::new(Chest::new(pos))));
            } else {
                self.structures
                    .push(RefCell::new(Box::new(Barrel::new(pos))));
            }
        }

        // Create trigger, TODO: Make an exit.
        self.trigger.clear();
        self.trigger.push(RefCell::new(Trigger {
            area: math::Rect {
                pos: (
                    1f32 * map::TILE_WIDTH as f32,
                    1f32 * map::TILE_HEIGHT as f32,
                ),
                size: (16f32, 16f32),
            },
            function: Box::new(|trigger: &Trigger, game: &Game, timestamp: f32| -> bool {
                if timestamp - trigger.last_triggered > 1000f32 {
                    // Check that all enemies are dead.
                    for enemy in &game.level.enemies {
                        if enemy.borrow().is_alive() {
                            return false;
                        }
                    }
                    game.level
                        .player
                        .borrow_mut()
                        .beam_to(AbsolutePosition(16f32, 32f32));

                    *game.change_level.borrow_mut() = true;
                    true
                } else {
                    false
                }
            }),
            last_triggered: 0f32,
        }));

        // Spawn enemies.
        // Boss enemies only at level 5, 10 etc
        let n_bosses = if level % 5 == 0 && level != 0 {
            level / 5
        } else {
            0
        };

        // No towers at the first boss lvl plz.
        let n_towers = if level != 5 { level / 3 } else { 0 };
        let n_bombs = if level != 5 { level / 2 } else { 0 };
        let n_stupid = (level + 1) / 2;

        for _ in 0..n_bosses {
            let pos = self
                .get_random_free_position((24f32, 16f32), true)
                .to_tuple();
            &self
                .enemies
                .push(RefCell::new(Box::new(Boss::new(pos, level))));
        }

        for _ in 0..n_bombs {
            let pos = self
                .get_random_free_position((12f32, 16f32), true)
                .to_tuple();
            &self
                .enemies
                .push(RefCell::new(Box::new(RunningBomb::new(pos, level))));
        }

        for _ in 0..n_towers {
            let pos = self
                .get_random_free_position((12f32, 16f32), true)
                .to_tuple();
            &self
                .enemies
                .push(RefCell::new(Box::new(Tower::new(pos, level))));
        }

        for _ in 0..n_stupid {
            let pos = self
                .get_random_free_position((12f32, 16f32), true)
                .to_tuple();

            &self
                .enemies
                .push(RefCell::new(Box::new(StupidEnemy::new(pos, level))));
        }
    }

    pub fn new() -> Self {
        let mut lvl = Self {
            map: RefCell::new(map::Map::new()),
            player: RefCell::new(player::Player::new()),

            enemies: Vec::with_capacity(100),
            // Reserve a lot of capacity during startup to reduce amount of
            // required reallocations.
            projectiles: RefCell::new(Vec::with_capacity(100)),
            structures: Vec::with_capacity(10),
            particles: RefCell::new(Vec::with_capacity(1000)),

            trigger: Vec::new(),

            misc: RefCell::new(Vec::new()),
            powerup: RefCell::new(Vec::new()),
        };

        lvl.generate_level(1);
        lvl
    }

    /// Update game logic, timestamp in ms.
    pub fn update(&mut self, input_state: &InputState, delta: f32) {
        // Update particles.
        {
            let particles = self.particles.borrow_mut();
            {
                let mut i = 0;
                while i != particles.len() {
                    particles[i].borrow_mut().update(&input_state, delta, &self);
                    if !particles[i].borrow().is_alive() {
                        particles.remove(i);
                    } else {
                        i += 1;
                    }
                }
            }
        }

        // Update projectiles.
        // TODO: Before cranking this up, the rocket projectile should only
        // spawn projectiles every N ms. Or at least N projectiles per ms.
        const N_PROJECTILE_SUBSTEPS: usize = 2;
        for _ in 0..N_PROJECTILE_SUBSTEPS {
            let mut projectiles = self.projectiles.borrow_mut();
            let mut i = 0;
            while i != projectiles.len() {
                projectiles[i].update(&input_state, delta / N_PROJECTILE_SUBSTEPS as f32, &self);

                // Check for collisions. O(n*m) algorithm, we might run into
                // performance issue at some point?

                if projectiles[i].can_hit(projectile::Team::Player) {
                    // Collides with player?
                    if collision::collides(&*projectiles[i], &*self.player.borrow()) {
                        self.player.borrow_mut().on_hit(&*projectiles[i], &self);
                        projectiles[i].destroy();
                    }
                }

                // Collides with a tile?
                {
                    let mut map = self.map.borrow_mut();
                    if let Some(t) = map.get_tile_at_mut(&projectiles[i].position().to_map()) {
                        if t.is_solid() {
                            t.on_hit(&*projectiles[i], &self);
                            projectiles[i].destroy();
                        }
                    }
                }

                // Collides with a structure?
                for x in 0..self.structures.len() {
                    if collision::collides(&*projectiles[i], &**self.structures[x].borrow()) {
                        self.structures[x]
                            .borrow_mut()
                            .on_hit(&*projectiles[i], &self);
                        projectiles[i].destroy();
                    }
                }

                // Collides with an enemy?
                if projectiles[i].can_hit(projectile::Team::Villain) {
                    for x in 0..self.enemies.len() {
                        if collision::collides(&*projectiles[i], &**self.enemies[x].borrow()) {
                            self.enemies[x].borrow_mut().on_hit(&*projectiles[i], &self);
                            projectiles[i].destroy();
                        }
                    }
                }

                if !projectiles[i].is_alive() {
                    projectiles.remove(i);
                } else {
                    i += 1;
                }
            }
        }

        // Update structures
        {
            let mut i = 0;
            while i != self.structures.len() {
                if !&self.structures[i].borrow().is_alive() {
                    &self.structures[i].borrow_mut().destroy(&self);
                    &self.structures.remove(i);
                } else {
                    &self.structures[i]
                        .borrow_mut()
                        .update(&input_state, delta, &self);
                    i += 1;
                }
            }
        }

        // Update misc (= bullet spawner right now)
        {
            let mut i = 0;
            let misc = self.misc.borrow_mut();
            while i != misc.len() {
                misc[i].update(&input_state, delta, &self);
                if !misc[i].is_alive() {
                    misc.remove(i);
                } else {
                    i += 1;
                }
            }
        }

        // Update enemies
        {
            let mut i = 0;
            while i != self.enemies.len() {
                &self.enemies[i]
                    .borrow_mut()
                    .update(&input_state, delta, &self);

                if !&self.enemies[i].borrow().is_alive() {
                    &self.enemies[i].borrow_mut().die(&self);
                    &self.enemies.remove(i);
                } else {
                    i += 1;
                }
            }

            if self.enemies.len() == 0 {
                self.map.borrow_mut().unlock_doors();
            }
        }
        // Update player
        {
            let player = self.player.borrow_mut();
            player.update(&input_state, delta, &self);

            // 'R'
            if input::key_just_pressed(0x52) {
                player.reload();
            }

            // 'Q'
            if input::key_just_pressed(0x51) {
                player.switch_weapon();
            }

            // space
            if input::key_just_pressed(0x20) {
                player.dodge_beam(&self.map.borrow_mut());
            }
        }

        {
            // Check whether player can pick up a powerup.
            let player_hitbox = self.player.borrow().hitbox();
            if player_hitbox.is_some() {
                let player_hitbox = player_hitbox.unwrap();
                let mut i = 0;
                let powerups = self.powerup.borrow_mut();
                while i != powerups.len() {
                    if powerups[i].hitbox().unwrap().overlaps(&player_hitbox) {
                        powerups[i].on_pickup(self);
                        powerups.remove(i);
                    } else {
                        i += 1;
                    }
                }
            }
        }
    }

    /// Render stuff.
    pub fn draw(&self, canvas: &mut Canvas, camera: &camera::Camera) {
        // Draw map in the background.
        self.map.borrow().draw(canvas, camera);

        for projectile in &*self.projectiles.borrow() {
            if projectile.is_alive() {
                projectile.draw(canvas, camera);
            }
        }

        for structure in &self.structures {
            if structure.borrow().is_alive() {
                structure.borrow().draw(canvas, camera);
            }
        }

        for enemy in &self.enemies {
            enemy.borrow_mut().draw(canvas, camera);
        }

        for misc in self.misc.borrow().iter() {
            misc.draw(canvas, camera);
        }

        for powerup in self.powerup.borrow().iter() {
            powerup.draw(canvas, camera);
        }

        self.player.borrow().draw(canvas, camera);

        for particle in &*self.particles.borrow() {
            particle.borrow().draw(canvas, camera);
        }
    }
}
