use camera;
use collision;
use level;
use projectile;
use {Canvas, InputState};

// All entities can collide.
pub trait Entity: collision::Collidable {
    /// Update the object, elapsed time since last call = delta (in ms).
    fn update(&mut self, input_state: &InputState, delta: f32, level: &level::Level);
    /// Draw the object to the canvas.
    fn draw(&self, canvas: &mut Canvas, camera: &camera::Camera);
    /// Got hit by a projectile.
    fn on_hit(&mut self, projectile: &projectile::Projectile, level: &level::Level);
}
