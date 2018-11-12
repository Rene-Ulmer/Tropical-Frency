use camera::Camera;
use color;
use common;
use gfx;
use gfx::Drawable;
use input;
use level;
use refcell::RefCell;
use resources;
use sound;

use InputState;

/// Contains the whole state of the game.
pub struct Game {
    pub camera: Camera,
    last_timestamp: Option<f32>,

    pub current_mouse_coords: common::ScreenPosition,

    music: sound::Music,

    pub level: level::Level,

    pub current_level: u8,
    pub change_level: RefCell<bool>,
}

impl Game {
    pub fn new() -> Self {
        Self {
            camera: Camera::new(),
            last_timestamp: None,

            current_mouse_coords: common::ScreenPosition(20, 20),

            music: sound::Music::new(),
            level: level::Level::new(),

            current_level: 1,
            change_level: RefCell::new(false),
        }
    }

    /// Update game logic, timestamp in ms.
    pub fn update(&mut self, input_state: &InputState, timestamp: f32) {
        let mut delta = match self.last_timestamp {
            Some(last_timestamp) => timestamp - last_timestamp,
            None => 0f32,
        };

        if *self.change_level.borrow() {
            // TODO: Do this only after teleport animation is done.
            self.level.generate_level((self.current_level + 1) as u32);
            self.current_level += 1;
            *self.change_level.borrow_mut() = false;
        }

        if delta > 500f32 {
            // Well, either it is lagging like shit, or the player pressed ESC
            // in the meantime, let's sanitize this delta.
            // Note: There will still be a bug with the trigger, as they store
            // the last timestamp themselves - whatever, it's a feature (TM).
            delta = 20f32;
        }
        self.last_timestamp = Some(timestamp);
        self.current_mouse_coords = input_state.mouse_pos;

        self.music.update(delta);

        // Check trigger
        for trigger in &self.level.trigger {
            trigger.borrow_mut().check(&self, timestamp);
        }
        self.level.update(input_state, delta);

        self.camera
            .center_around(self.level.player.borrow().get_center_position().round());

        let mouse_absolute = input_state.mouse_pos.to_absolute(Some(self.camera.get()));

        // Ugly, move this back into level? or more like player? I don't know.
        if input::mouse_just_pressed(0) {
            let center = self.level.player.borrow().get_center_position();

            self.level
                .player
                .borrow_mut()
                .shoot(&center, &mouse_absolute, &self.level);
        }

        self.level
            .player
            .borrow_mut()
            .update_facing(&mouse_absolute);
    }

    /// Render stuff.
    pub fn draw(&self, canvas: &mut gfx::Canvas) {
        self.level.draw(canvas, &self.camera);

        gfx::draw_line(
            canvas,
            self.level
                .player
                .borrow()
                .get_center_position()
                .to_screen(&self.camera),
            self.current_mouse_coords,
            color::DARK_RED,
        );

        let mut draw_number = |tileset: &gfx::Tileset, n: u8, idx: isize, idy: isize| {
            let pos = common::ScreenPosition(6 * idx, 6 * idy);
            tileset
                .get_tile(((n as usize + 9) % 10, 0))
                .unwrap()
                .draw(pos, canvas);
        };

        // Draw current level.
        {
            let tileset_data = gfx::BitData::new(
                resources::TILESET_NUMBERS,
                (50, 8),
                1,
                &[None, Some(color::RED)],
            );
            let tileset = gfx::Tileset::new(&tileset_data, (5, 8));
            draw_number(&tileset, self.current_level / 10, 0, 0);
            draw_number(&tileset, self.current_level, 1, 0);
        }

        // Draw enemy count
        {
            let tileset_data = gfx::BitData::new(
                resources::TILESET_NUMBERS,
                (50, 8),
                1,
                &[None, Some(color::SILVER)],
            );
            let tileset = gfx::Tileset::new(&tileset_data, (5, 8));
            let elen = self.level.enemies.len();
            draw_number(&tileset, (elen / 10) as u8, 0, 2);
            draw_number(&tileset, elen as u8, 1, 2);
        }
    }
}
