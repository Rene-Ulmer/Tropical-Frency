use camera::Camera;
use collision::Collidable;
use common::AbsolutePosition;
use entity::Entity;
use gfx;
use gfx::Drawable;
use level;
use math;
use projectile;
use InputState;

pub struct SimpleProjectile {
    pos: AbsolutePosition,
    velocity: (f32, f32),
    alive: bool,
    time_to_live: f32,
    team: projectile::Team,

    color: u32,
}

impl SimpleProjectile {
    pub fn new_from_absolute_coords(
        src: &AbsolutePosition,
        dst: &AbsolutePosition,
        speed: f32,
        time_to_live: f32,
        team: projectile::Team,
        color: u32,
    ) -> Self {
        // Normalize velocity
        let vel_x = (dst.0 - src.0) as f32;
        let vel_y = (dst.1 - src.1) as f32;
        let (vel_x, vel_y) = math::normalize((vel_x, vel_y));

        Self {
            pos: *src,
            velocity: ((vel_x * speed), (vel_y * speed)),
            alive: true,
            time_to_live,
            team,
            color,
        }
    }
}

impl projectile::Projectile for SimpleProjectile {
    fn destroy(&mut self) {
        self.alive = false;
        self.time_to_live = -1f32;
    }

    fn is_alive(&self) -> bool {
        self.alive && self.pos.0 >= 0f32 && self.pos.1 >= 0f32
    }

    fn velocity(&self) -> (f32, f32) {
        self.velocity
    }

    fn position(&self) -> AbsolutePosition {
        self.pos
    }

    fn damage(&self) -> isize {
        1
    }

    fn onhit_recoil(&self) -> f32 {
        0.05f32
    }

    fn can_hit(&self, team: projectile::Team) -> bool {
        match (team, &self.team) {
            (projectile::Team::Player, projectile::Team::Player) => false,
            (projectile::Team::Villain, projectile::Team::Villain) => false,
            _ => true,
        }
    }

    fn particle_multiplier(&self) -> u32 {
        1
    }
}

impl Entity for SimpleProjectile {
    fn update(&mut self, _: &InputState, delta: f32, _game: &level::Level) {
        self.pos.0 += self.velocity.0 * (delta / 1000f32);
        self.pos.1 += self.velocity.1 * (delta / 1000f32);
        if self.alive {
            self.time_to_live -= delta;
        }
        if self.time_to_live < 0f32 {
            self.alive = false;
        }
    }

    /// Draw the object to the canvas.
    fn draw(&self, canvas: &mut gfx::Canvas, camera: &Camera) {
        let color_func = |dist: f32| {
            if dist < 1.0f32 {
                Some(self.color)
            } else {
                None
            }
        };
        let bullet_img_data = gfx::Circle::new(4, &color_func);
        let bullet_img = gfx::Sprite::new(&bullet_img_data);

        let pos = self.pos.to_screen(camera);
        bullet_img.draw(pos, canvas);
    }

    fn on_hit(&mut self, _: &projectile::Projectile, _: &level::Level) {
        // Well, we can't be hit so this is meaningless.
    }
}

impl Collidable for SimpleProjectile {
    fn hitbox(&self) -> Option<math::Rect> {
        Some(math::Rect {
            pos: (self.pos.0, self.pos.1),
            size: (4f32, 4f32),
        })
    }
}
