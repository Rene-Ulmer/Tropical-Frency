use gfx::ImageSource;

pub struct RGBData {
    data: &'static [u8],
    size: (usize, usize),
}

impl RGBData {
    pub fn new(data: &'static [u8], size: (usize, usize)) -> Self {
        // TODO: Length check data.len() == size.0 * size.1 * 3
        RGBData { data, size }
    }
}

impl ImageSource for RGBData {
    fn get_pixel_rgb(&self, position: (usize, usize)) -> Option<u32> {
        let rgb: u32 = ((self.data[(self.size.0 * position.1 + position.0) * 3 + 0] as u32) << 16)
            | ((self.data[(self.size.0 * position.1 + position.0) * 3 + 1] as u32) << 8)
            | self.data[(self.size.0 * position.1 + position.0) * 3 + 2] as u32;
        Some(rgb)
    }

    fn width(&self) -> usize {
        self.size.0
    }

    fn height(&self) -> usize {
        self.size.1
    }
}
