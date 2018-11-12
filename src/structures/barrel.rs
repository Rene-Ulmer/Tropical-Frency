use alloc::boxed::Box;
use camera::Camera;
use collision::Collidable;
use color;
use common::AbsolutePosition;
use effects;
use entity::Entity;
use filter;
use gfx;
use gfx::Canvas;
use gfx::Drawable;
use level;
use math::Rect;
use particle;
use projectile;
use random;
use refcell::RefCell;
use resources;
use structures::Structure;
use InputState;

pub struct Barrel {
    obj: Rect,
    hp: isize,
    spawn_next_particles_at: f32,
}

impl Barrel {
    pub fn new(pos: AbsolutePosition) -> Self {
        Barrel {
            obj: Rect {
                pos: pos.to_tuple(),
                size: (16f32, 16f32),
            },
            hp: 5,
            spawn_next_particles_at: 0f32,
        }
    }
}

impl Entity for Barrel {
    fn update(&mut self, _input: &InputState, delta: f32, level: &level::Level) {
        if self.spawn_next_particles_at < delta {
            // spawn fire particles
            let start = AbsolutePosition::from_f32(((random::rand() % 8) as f32 + 4f32, 4f32));

            // Different speed categories.
            const TYPES: [(f32, f32); 5] = [
                (0.1f32, 0.3f32),
                (0.3f32, 0.5f32),
                (0.5f32, 0.6f32),
                (0.7f32, 0.8f32),
                (1.0f32, 1.0f32),
            ];

            let t = TYPES[(random::rand() % 5) as usize];
            let speed = t.0;
            let lifetime = t.1;

            level.particles.borrow_mut().push(RefCell::new(Box::new(
                particle::DecayParticle::new(
                    self.position().add(start),
                    (0f32, -10f32 * speed),
                    lifetime,
                    [
                        color::ORANGE_RED,
                        color::ORANGE,
                        color::ORANGE_YELLOW,
                        color::SUN_YELLOW,
                        color::BRIGHT_YELLOW,
                    ],
                    0f32,
                ),
            )));

            self.spawn_next_particles_at = 50f32;
        } else {
            self.spawn_next_particles_at -= delta;
        }
    }

    fn draw(&self, canvas: &mut Canvas, camera: &Camera) {
        let tileset_data = gfx::RGBData::new(resources::TILESET, resources::TILESET_SIZE);;
        let tileset = gfx::Tileset::new(&tileset_data, (16, 16));

        let tile = tileset.get_tile((2, 0)).unwrap();
        let img = filter::TransparencyFilter::new(&tile, color::PINK);
        let sprite = gfx::Sprite::new(&img);

        sprite.draw(self.position().to_screen(camera), canvas);
    }

    fn on_hit(&mut self, projectile: &projectile::Projectile, _level: &level::Level) {
        self.hp -= projectile.damage();
    }
}

impl Structure for Barrel {
    fn destroy(&mut self, level: &level::Level) {
        // TODO: scale with difficulty.
        level.misc.borrow_mut().push(effects::BulletSpawner::new(
            &self.position().add(AbsolutePosition(6f32, 6f32)),
            19,     // bullets per pulse
            11,     // # pulses
            100f32, // time between pulses
            spawn_bullet,
        ));
    }
    fn is_alive(&self) -> bool {
        self.hp > 0
    }

    fn position(&self) -> AbsolutePosition {
        AbsolutePosition::from_f32(self.obj.pos)
    }
}

impl Collidable for Barrel {
    fn hitbox(&self) -> Option<Rect> {
        Some(Rect {
            pos: (self.obj.pos.0 + 2f32, self.obj.pos.1 + 2f32),
            size: (12f32, 13f32),
        })
    }
}

fn spawn_bullet(level: &level::Level, from: &AbsolutePosition, to: &AbsolutePosition) {
    const COLORS: [u32; 6] = [
        color::BOMB_ORANGE,
        color::ORANGE_RED,
        color::ORANGE,
        color::ORANGE_YELLOW,
        color::SUN_YELLOW,
        color::BRIGHT_YELLOW,
    ];

    let color = COLORS[random::rand() as usize % COLORS.len()];
    level.projectiles.borrow_mut().push(Box::new(
        projectile::SimpleProjectile::new_from_absolute_coords(
            from,
            to,
            100f32,
            350f32,
            projectile::Team::Neutral,
            color,
        ),
    ));
}
