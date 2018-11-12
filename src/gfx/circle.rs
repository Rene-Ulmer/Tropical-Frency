use gfx::ImageSource;

pub struct Circle<'a> {
    diameter: usize,
    color_func: &'a Fn(f32) -> Option<u32>,
}

impl<'a> Circle<'a> {
    pub fn new(diameter: usize, color_func: &'a Fn(f32) -> Option<u32>) -> Self {
        Self {
            diameter,
            color_func,
        }
    }
}

impl<'a> ImageSource for Circle<'a> {
    fn get_pixel_rgb(&self, position: (usize, usize)) -> Option<u32> {
        let dx = position.0 as isize - self.diameter as isize / 2;
        let dy = position.1 as isize - self.diameter as isize / 2;

        let dist = (dx * dx + dy * dy) as f32
            / ((self.diameter as isize * self.diameter as isize / 4) as f32);
        (self.color_func)(dist)
    }

    fn width(&self) -> usize {
        self.diameter
    }

    fn height(&self) -> usize {
        self.diameter
    }
}
