use gfx::ImageSource;

pub struct TruncatingFilter<'a> {
    inner: &'a ImageSource,
    amount_x: usize,
    amount_y: usize,
}

impl<'a> TruncatingFilter<'a> {
    pub fn new(source: &'a ImageSource, amount_x: usize, amount_y: usize) -> Self {
        Self {
            inner: source,
            amount_x,
            amount_y,
        }
    }
}

impl<'a> ImageSource for TruncatingFilter<'a> {
    fn get_pixel_rgb(&self, position: (usize, usize)) -> Option<u32> {
        if position.0 <= self.amount_x || self.width() - position.0 - 1 <= self.amount_x {
            None
        } else if position.1 <= self.amount_y || self.height() - position.1 - 1 <= self.amount_y {
            None
        } else {
            self.inner.get_pixel_rgb(position)
        }
    }

    fn width(&self) -> usize {
        self.inner.width()
    }

    fn height(&self) -> usize {
        self.inner.height()
    }
}
