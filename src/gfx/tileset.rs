use common::ScreenPosition;
use gfx::canvas::Canvas;
use gfx::Drawable;
use gfx::ImageSource;

/// Tileset.
pub struct Tileset<'a> {
    /// Pointer to the image data.
    image: &'a ImageSource,

    /// Dimensions of each tile.
    tile_size: (usize, usize),
}

impl<'a> Tileset<'a> {
    pub fn new(image: &'a ImageSource, tile_size: (usize, usize)) -> Self {
        if image.width() % tile_size.0 != 0 || image.height() % tile_size.1 != 0 {
            // size should be divisible by tile_size.
            unreachable!();
        }

        Self { image, tile_size }
    }

    pub fn tiles_per_row(&self) -> usize {
        self.image.width() / self.tile_size.0
    }

    pub fn tiles_per_column(&self) -> usize {
        self.image.height() / self.tile_size.1
    }

    pub fn get_tile(&self, index: (usize, usize)) -> Result<TilesetTile, ()> {
        if index.0 < self.tiles_per_row() && index.1 < self.tiles_per_column() {
            Ok(TilesetTile {
                tileset: self,
                index,
            })
        } else {
            Err(())
        }
    }
}

/// A single tile out of a tileset.
pub struct TilesetTile<'a> {
    tileset: &'a Tileset<'a>,
    index: (usize, usize),
}

impl<'a> Drawable for TilesetTile<'a> {
    fn draw(&self, position: ScreenPosition, canvas: &mut Canvas) {
        for tile_x in 0..self.tileset.tile_size.0 {
            for tile_y in 0..self.tileset.tile_size.1 {
                let target_x = tile_x.wrapping_add(position.0 as usize);
                let target_y = tile_y.wrapping_add(position.1 as usize);

                let source_x = tile_x + self.index.0 * self.tileset.tile_size.0;
                let source_y = tile_y + self.index.1 * self.tileset.tile_size.1;

                if let Some(color) = self.tileset.image.get_pixel_rgb((source_x, source_y)) {
                    // We don't care about errors here.
                    canvas.set_pixel(target_x, target_y, color).ok();
                }
            }
        }
    }
}

impl<'a> ImageSource for TilesetTile<'a> {
    fn get_pixel_rgb(&self, position: (usize, usize)) -> Option<u32> {
        let tile_x = position.0;
        let tile_y = position.1;
        if tile_x < self.tileset.tile_size.0 && tile_y < self.tileset.tile_size.1 {
            let source_x = tile_x + self.index.0 * self.tileset.tile_size.0;
            let source_y = tile_y + self.index.1 * self.tileset.tile_size.1;

            self.tileset.image.get_pixel_rgb((source_x, source_y))
        } else {
            None
        }
    }

    fn width(&self) -> usize {
        self.tileset.tile_size.0
    }

    fn height(&self) -> usize {
        self.tileset.tile_size.1
    }
}
