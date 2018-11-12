use gfx::ImageSource;

pub struct BitData<'a> {
    data: &'static [u8],
    size: (usize, usize),
    n_bits: usize,
    colors: &'a [Option<u32>],
}

impl<'a> BitData<'a> {
    pub fn new(
        data: &'static [u8],
        size: (usize, usize),
        n_bits: usize,
        colors: &'a [Option<u32>],
    ) -> Self {
        if data.len() < size.0 * size.1 * n_bits / 8 {
            panic!("Invalid buffer length");
        }

        if n_bits & (n_bits - 1) != 0 {
            panic!("n_bits not a power of 2, not supported");
        }

        if n_bits == 0 || n_bits > 8 {
            panic!("Invalid n_bits");
        }

        if 1 << n_bits != colors.len() {
            panic!("Color table incorrect");
        }

        BitData {
            data,
            size,
            n_bits,
            colors,
        }
    }
}

impl<'a> ImageSource for BitData<'a> {
    fn get_pixel_rgb(&self, position: (usize, usize)) -> Option<u32> {
        let offset = self.size.0 * position.1 + position.0;
        let data_idx = offset / (8 / self.n_bits);
        let data_off = offset % (8 / self.n_bits);
        let mut mask = 0;
        for _ in 0..self.n_bits {
            mask <<= 1;
            mask |= 1;
        }
        let px_val = (self.data[data_idx] >> (7 - (data_off))) & mask;
        self.colors[px_val as usize]
    }

    fn width(&self) -> usize {
        self.size.0
    }

    fn height(&self) -> usize {
        self.size.1
    }
}
