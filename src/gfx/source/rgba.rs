use gfx::ImageSource;

pub struct RGBAData {
    data: &'static [u8],
    size: (usize, usize),
}

impl RGBAData {
    pub fn new(data: &'static [u8], size: (usize, usize)) -> Self {
        // TODO: Length check data.len() == size.0 * size.1 * 4
        RGBAData { data, size }
    }
}
//0 b
//8 g
//16 r
//24 a
impl ImageSource for RGBAData {
    fn get_pixel_rgb(&self, position: (usize, usize)) -> Option<u32> {
        let rgb: u32 = (self.data[(self.size.0 * position.1 + position.0) * 4 + 1] as u32)
            | ((self.data[(self.size.0 * position.1 + position.0) * 4 + 1] as u32) << 8)
            | (self.data[(self.size.0 * position.1 + position.0) * 4 + 1] as u32) << 16;
        Some(rgb)
    }

    fn width(&self) -> usize {
        self.size.0
    }

    fn height(&self) -> usize {
        self.size.1
    }
}
