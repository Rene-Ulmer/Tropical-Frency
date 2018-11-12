use alloc::boxed::Box;
use game::Game;
use math;

pub struct Trigger {
    pub area: math::Rect,
    pub function: Box<Fn(&Trigger, &Game, f32) -> bool>,

    // Timestamp in ms - has to be set by the trigger!
    pub last_triggered: f32,
}

impl Trigger {
    pub fn check(&mut self, game: &Game, timestamp: f32) {
        let player_pos = game.level.player.borrow().get_center_position().to_tuple();
        if self.area.contains(player_pos) {
            if (self.function)(self, game, timestamp) {
                self.last_triggered = timestamp;
            }
        }
    }
}
