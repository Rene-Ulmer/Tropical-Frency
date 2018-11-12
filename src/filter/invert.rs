use gfx::ImageSource;

pub struct InvertingFilter<'a> {
    inner: &'a ImageSource,
}

impl<'a> InvertingFilter<'a> {
    pub fn new(source: &'a ImageSource) -> Self {
        Self { inner: source }
    }
}

impl<'a> ImageSource for InvertingFilter<'a> {
    fn get_pixel_rgb(&self, position: (usize, usize)) -> Option<u32> {
        self.inner.get_pixel_rgb(position).map(|v| 0xFFFFFF - v)
    }

    fn width(&self) -> usize {
        self.inner.width()
    }

    fn height(&self) -> usize {
        self.inner.height()
    }
}
