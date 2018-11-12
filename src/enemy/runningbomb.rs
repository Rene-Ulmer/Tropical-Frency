use alloc::boxed::Box;
use camera::Camera;
use collision::Collidable;
use color;
use common::AbsolutePosition;
use effects;
use enemy::Enemy;
use entity::Entity;
use filter;
use gfx;
use gfx::draw_line;
use gfx::Canvas;
use gfx::Drawable;
use level;
use math;
use math::Rect;
use misc::inside_vision;
use movement;
use particle;
use powerup;
use projectile;
use random;
use resources;
use InputState;

enum Status {
    Walking,
    Idle,
}

pub struct RunningBomb {
    obj: Rect,

    // action
    status: Status,
    ready: bool,
    time_until_ready: f32,

    // movement
    current_velocity: (f32, f32),
    knockback_velocity: (f32, f32),
    speed: f32,

    // for flipping the image every now and then.
    movement_counter: f32,

    // health related stuff
    hp: isize,
}

impl RunningBomb {
    pub fn new(pos: (f32, f32), level: u32) -> Self {
        // Put some meaningful default values here.
        Self {
            obj: Rect {
                pos: pos,
                size: (16f32, 16f32),
            },

            status: Status::Walking,
            ready: false,
            time_until_ready: 1000f32,

            current_velocity: (0f32, 0f32),
            knockback_velocity: (0f32, 0f32),
            speed: 40f32 + level as f32,

            movement_counter: 0f32,

            hp: 1 + level as isize / 3,
        }
    }

    pub fn get_center_position(&self) -> AbsolutePosition {
        AbsolutePosition::from_f32((
            self.obj.pos.0 + self.obj.size.0 / 2f32,
            self.obj.pos.1 + self.obj.size.1 / 2f32,
        ))
    }
}

impl Collidable for RunningBomb {
    fn hitbox(&self) -> Option<Rect> {
        let mut hitbox = self.obj.clone();
        hitbox.pos.0 += 3f32;
        hitbox.size.0 -= 6f32;
        Some(hitbox)
    }
}

impl Entity for RunningBomb {
    fn update(&mut self, _input: &InputState, delta: f32, level: &level::Level) {
        if self.time_until_ready > 0f32 {
            self.time_until_ready -= delta;
        }
        match self.status {
            Status::Idle => {
                if self.time_until_ready <= 0f32 {
                    self.ready = true;
                }
            }
            Status::Walking => {
                if self.time_until_ready <= 0f32 {
                    self.ready = true;
                }
            }
        }
        if self.ready {
            if inside_vision(
                &level.player.borrow().get_center_position(),
                &self.get_center_position(),
                &level.map.borrow(),
            ) {
                self.status = Status::Walking;
            } else {
                self.status = Status::Idle;
            }
            self.time_until_ready = match self.status {
                Status::Idle => 500f32,
                Status::Walking => 1000f32,
            };
            self.ready = false;
        }

        // explode near player
        let mut distance_to_explode = 25f32;
        distance_to_explode *= distance_to_explode;
        let mut distance_between_player = self
            .position()
            .sub(*&level.player.borrow().get_center_position());
        distance_between_player.0 *= distance_between_player.0;
        distance_between_player.1 *= distance_between_player.1;
        if distance_between_player.0 + distance_between_player.1 < distance_to_explode {
            self.hp = 0;
        }

        match self.status {
            Status::Walking => {
                movement::move_to_player(
                    &self.hitbox().unwrap(),
                    &mut self.current_velocity,
                    self.speed,
                    level,
                );
            }
            _ => {}
        }
        movement::apply_force(
            &mut self.obj.pos,
            (16f32, 16f32),
            &mut self.current_velocity,
            &mut self.knockback_velocity,
            delta,
            level,
        );

        if math::len_vec2(self.current_velocity) > 0.5f32 {
            self.movement_counter += delta;
        }

        // bomb particles

        let start = AbsolutePosition::from_f32((11f32, 6f32));
        particle::spawn_directional_particles(
            level,
            self.position().add(start),
            20f32,
            180f32,
            color::BOMB_PARTICLES,
            1,
        );
    }

    fn draw(&self, canvas: &mut Canvas, camera: &Camera) {
        let tileset_data = gfx::RGBData::new(resources::TILESET, resources::TILESET_SIZE);;
        let tileset = gfx::Tileset::new(&tileset_data, (16, 16));

        let tile = tileset.get_tile((5, 0)).unwrap();
        let img = filter::TransparencyFilter::new(&tile, color::PINK);

        let flip = if (self.movement_counter / 500f32) as usize % 2 == 0 {
            filter::MirrorMode::None
        } else {
            filter::MirrorMode::Vertical
        };
        let img = filter::MirroringFilter::new(&img, flip);
        let sprite = gfx::Sprite::new(&img);

        sprite.draw(self.position().to_screen(camera), canvas);

        // draw bomb
        let start = AbsolutePosition::from_f32((11f32, 7f32));
        let end = AbsolutePosition::from_f32((11f32, 10f32));
        draw_line(
            canvas,
            self.position().add(start).to_screen(camera),
            self.position().add(end).to_screen(camera),
            color::BOMB_ORANGE,
        );
    }

    fn on_hit(&mut self, projectile: &projectile::Projectile, level: &level::Level) {
        self.hp -= projectile.damage();

        let vel = projectile.velocity();
        self.knockback_velocity.0 += vel.0 * projectile.onhit_recoil();
        self.knockback_velocity.1 += vel.1 * projectile.onhit_recoil();

        effects::on_hit_effect(
            projectile,
            level,
            self.get_center_position(),
            1,
            color::DARK_GREEN,
            color::GREEN,
        );
    }
}

impl Enemy for RunningBomb {
    fn die(&mut self, level: &level::Level) {
        level.misc.borrow_mut().push(effects::BulletSpawner::new(
            &self.get_center_position(),
            7,      // bullets per pulse
            8,      // # pulses
            200f32, // time between pulses
            spawn_bullet,
        ));

        powerup::maybe_drop_powerup(self.position(), 10, level);
    }

    fn is_alive(&self) -> bool {
        self.hp > 0
    }

    fn position(&self) -> AbsolutePosition {
        AbsolutePosition::from_f32(self.obj.pos)
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
            50f32,
            500f32,
            projectile::Team::Neutral,
            color,
        ),
    ));
}
