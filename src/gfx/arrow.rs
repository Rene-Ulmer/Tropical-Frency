use gfx::ImageSource;

pub struct Arrow {
    size: (usize, usize),
}

impl Arrow {
    pub fn new(size: (usize, usize)) -> Self {
        Self { size }
    }
}

impl ImageSource for Arrow {
    fn get_pixel_rgb(&self, position: (usize, usize)) -> Option<u32> {
        // Create an arrow pointing up.
        // h in the middle and v between {20% and 80%} -> set
        // v between {20% and 50%} -> h +- (v-20) set.

        let is_h_middle = position.0 == self.size.0 / 2;

        let p1 = position.1 as f32;
        let s1 = self.size.1 as f32;
        let is_between_20_80 = p1 >= s1 * 0.20f32 && p1 < s1 * 0.80f32;
        let is_between_20_50 = p1 >= s1 * 0.20f32 && p1 < s1 * 0.50f32;

        let top20 = s1 * 0.20f32;
        let dt20 = p1 - top20;

        let px = is_between_20_80 && is_h_middle
            || is_between_20_50
                && (position.0 == (self.size.0 / 2) + dt20 as usize
                    || position.0 == (self.size.0 / 2) - dt20 as usize);
        if px {
            Some(0)
        } else {
            Some(0xFFFFFF)
        }
    }

    fn width(&self) -> usize {
        self.size.0
    }

    fn height(&self) -> usize {
        self.size.1
    }
}
