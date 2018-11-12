use common::ScreenPosition;

mod arrow;
mod canvas;
mod circle;
mod primitives;
mod source;
mod sprite;
mod tileset;

pub use self::arrow::Arrow;
pub use self::canvas::Canvas;
pub use self::circle::Circle;
pub use self::primitives::{draw_line, draw_rect, fill_rect};
pub use self::source::*;
pub use self::sprite::Sprite;
pub use self::tileset::{Tileset, TilesetTile};

/// Contains methods to draw something on something else.
pub trait Drawable {
    /// Draw ourselves to the canvas at the provided coordinates.
    fn draw(&self, position: ScreenPosition, canvas: &mut Canvas);
}

/// Methods that can be used from image sources (e.g. sprite data).
pub trait ImageSource {
    /// Read a single pixel color value.
    // TODO: Option<X> increases size, maybe reserve special value?
    fn get_pixel_rgb(&self, position: (usize, usize)) -> Option<u32>;

    /// Get width of image source.
    fn width(&self) -> usize;

    /// Get height of image source.
    fn height(&self) -> usize;
}
