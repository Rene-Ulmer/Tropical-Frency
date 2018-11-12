use gfx::ImageSource;

pub struct ScalingFilter<'a> {
    inner: &'a ImageSource,
    scaling: f32,
}

impl<'a> ScalingFilter<'a> {
    pub fn new(source: &'a ImageSource, scaling: f32) -> Self {
        Self {
            inner: source,
            scaling,
        }
    }
}

impl<'a> ImageSource for ScalingFilter<'a> {
    fn get_pixel_rgb(&self, position: (usize, usize)) -> Option<u32> {
        let position = if self.scaling > 0f32 {
            // TODO: Is the casting to f32 necessary?
            (
                (position.0 as f32 / self.scaling) as usize,
                (position.1 as f32 / self.scaling) as usize,
            )
        } else {
            (
                (position.0 as f32 * self.scaling) as usize,
                (position.1 as f32 * self.scaling) as usize,
            )
        };
        self.inner.get_pixel_rgb(position)
    }

    fn width(&self) -> usize {
        (self.inner.width() as f32 * self.scaling) as usize
    }

    fn height(&self) -> usize {
        (self.inner.height() as f32 * self.scaling) as usize
    }
}
