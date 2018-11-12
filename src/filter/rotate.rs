use gfx::ImageSource;

#[derive(Copy, Clone)]
pub enum Rotation {
    Rotate0,
    Rotate90,
    Rotate180,
    Rotate270,
}

pub struct RotatingFilter<'a> {
    inner: &'a ImageSource,
    rotation: Rotation,
}

impl<'a> RotatingFilter<'a> {
    pub fn new(source: &'a ImageSource, rotation: Rotation) -> Self {
        Self {
            inner: source,
            rotation,
        }
    }
}

impl<'a> ImageSource for RotatingFilter<'a> {
    fn get_pixel_rgb(&self, position: (usize, usize)) -> Option<u32> {
        let position = match self.rotation {
            Rotation::Rotate0 => position,
            Rotation::Rotate90 => (position.1, self.width() - position.0 - 1),
            Rotation::Rotate180 => (
                self.width() - position.0 - 1,
                self.height() - position.1 - 1,
            ),
            Rotation::Rotate270 => (self.height() - position.1 - 1, position.0),
        };
        self.inner.get_pixel_rgb(position)
    }

    fn width(&self) -> usize {
        match self.rotation {
            Rotation::Rotate0 | Rotation::Rotate180 => self.inner.width(),
            Rotation::Rotate90 | Rotation::Rotate270 => self.inner.height(),
        }
    }
    fn height(&self) -> usize {
        match self.rotation {
            Rotation::Rotate0 | Rotation::Rotate180 => self.inner.height(),
            Rotation::Rotate90 | Rotation::Rotate270 => self.inner.width(),
        }
    }
}
