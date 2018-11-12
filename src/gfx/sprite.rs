use common::ScreenPosition;
use gfx::canvas::Canvas;
use gfx::Drawable;
use gfx::ImageSource;

/// Single, static image.
pub struct Sprite<'a> {
    image: &'a ImageSource,
}

impl<'a> Sprite<'a> {
    pub fn new(image_source: &'a ImageSource) -> Self {
        Self {
            image: image_source,
        }
    }
}

impl<'a> Drawable for Sprite<'a> {
    fn draw(&self, position: ScreenPosition, canvas: &mut Canvas) {
        for sprite_x in 0..self.image.width() {
            let x = sprite_x.wrapping_add(position.0 as usize);
            for sprite_y in 0..self.image.height() {
                let y = sprite_y.wrapping_add(position.1 as usize);

                if let Some(color) = self.image.get_pixel_rgb((sprite_x, sprite_y)) {
                    canvas.set_pixel(x, y, color).ok();
                }
            }
        }
    }
}
