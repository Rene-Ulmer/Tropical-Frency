use gfx::ImageSource;

#[derive(Copy, Clone)]
pub enum MirrorMode {
    None,
    Vertical,
    Horizontal,
}

pub struct MirroringFilter<'a> {
    inner: &'a ImageSource,
    mode: MirrorMode,
}

impl<'a> MirroringFilter<'a> {
    pub fn new(source: &'a ImageSource, mode: MirrorMode) -> Self {
        Self {
            inner: source,
            mode,
        }
    }
}

impl<'a> ImageSource for MirroringFilter<'a> {
    fn get_pixel_rgb(&self, position: (usize, usize)) -> Option<u32> {
        let position = match self.mode {
            MirrorMode::Vertical => (self.width() - position.0 - 1, position.1),
            MirrorMode::Horizontal => (position.0, self.height() - position.1 - 1),
            MirrorMode::None => position,
        };
        self.inner.get_pixel_rgb(position)
    }

    fn width(&self) -> usize {
        self.inner.width()
    }
    fn height(&self) -> usize {
        self.inner.height()
    }
}
