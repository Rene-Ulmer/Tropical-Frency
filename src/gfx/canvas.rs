use alloc::vec::Vec;

/// Represents our target canvas.
pub struct Canvas<'a> {
    buffer: &'a mut [u8],
    pub dimensions: (usize, usize),

    colors_used: Vec<u32>,
}

impl<'a> Canvas<'a> {
    pub fn new(buffer: &'a mut [u8], dimensions: (usize, usize)) -> Self {
        // Make sure that alpha = 0xFF.
        for x in 0..dimensions.0 {
            for y in 0..dimensions.1 {
                buffer[(y * dimensions.0 + x) * 4 + 3] = 0xFF;
            }
        }

        Self {
            buffer,
            dimensions,
            colors_used: Vec::new(),
        }
    }

    fn use_color(&mut self, c: u32) {
        for used in &self.colors_used {
            if c == *used {
                return;
            }
        }
        self.colors_used.push(c);
    }

    /// Sets a single pixel on the canvas.
    pub fn set_pixel(&mut self, x: usize, y: usize, rgb: u32) -> Result<(), ()> {
        self.use_color(rgb);
        if x < self.dimensions.0 && y < self.dimensions.1 {
            self.buffer[(y * self.dimensions.0 + x) * 4 + 0] = (rgb >> 16) as u8;
            self.buffer[(y * self.dimensions.0 + x) * 4 + 1] = (rgb >> 8) as u8;
            self.buffer[(y * self.dimensions.0 + x) * 4 + 2] = rgb as u8;
            Ok(())
        } else {
            Err(())
        }
    }

    /// Clears the whole canvas with the provided color.
    pub fn clear(&mut self, rgb: u32) {
        self.use_color(rgb);
        for x in 0..self.dimensions.0 {
            for y in 0..self.dimensions.1 {
                self.buffer[(y * self.dimensions.0 + x) * 4 + 0] = (rgb >> 16) as u8;
                self.buffer[(y * self.dimensions.0 + x) * 4 + 1] = (rgb >> 8) as u8;
                self.buffer[(y * self.dimensions.0 + x) * 4 + 2] = rgb as u8;
            }
        }
    }

    pub fn used_colors(&self) -> usize {
        self.colors_used.len()
    }
}
