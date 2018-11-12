use gfx::ImageSource;

pub struct TransparencyFilter<'a> {
    inner: &'a ImageSource,
    color: u32,
}

impl<'a> TransparencyFilter<'a> {
    pub fn new(source: &'a ImageSource, color: u32) -> Self {
        Self {
            inner: source,
            color,
        }
    }
}

impl<'a> ImageSource for TransparencyFilter<'a> {
    fn get_pixel_rgb(&self, position: (usize, usize)) -> Option<u32> {
        match self.inner.get_pixel_rgb(position) {
            Some(r) => if r == self.color {
                None
            } else {
                Some(r)
            },
            None => None,
        }
    }

    fn width(&self) -> usize {
        self.inner.width()
    }

    fn height(&self) -> usize {
        self.inner.height()
    }
}
