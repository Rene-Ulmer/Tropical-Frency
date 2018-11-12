use gfx::ImageSource;

pub struct MonoFilter<'a> {
    inner: &'a ImageSource,
    color: u32,
}

impl<'a> MonoFilter<'a> {
    pub fn new(source: &'a ImageSource, color: u32) -> Self {
        Self {
            inner: source,
            color,
        }
    }
}

impl<'a> ImageSource for MonoFilter<'a> {
    fn get_pixel_rgb(&self, position: (usize, usize)) -> Option<u32> {
        self.inner.get_pixel_rgb(position).map(|_| self.color)
    }

    fn width(&self) -> usize {
        self.inner.width()
    }

    fn height(&self) -> usize {
        self.inner.height()
    }
}
