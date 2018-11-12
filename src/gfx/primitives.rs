use common::ScreenPosition;
use gfx::canvas::Canvas;

/// Draw a rectangle (no fill).
pub fn draw_rect(canvas: &mut Canvas, pos: ScreenPosition, size: (usize, usize), color: u32) {
    for x_offset in 0..size.0 {
        let x = pos.0 as usize + x_offset;
        canvas.set_pixel(x, pos.1 as usize, color).ok();
        canvas.set_pixel(x, pos.1 as usize + size.1 - 1, color).ok();
    }
    for y_offset in 0..size.1 {
        let y = pos.1 as usize + y_offset;
        canvas.set_pixel(pos.0 as usize, y, color).ok();
        canvas.set_pixel(pos.0 as usize + size.0 - 1, y, color).ok();
    }
}

/// Fill a rectangle (fill).
pub fn fill_rect(canvas: &mut Canvas, pos: ScreenPosition, size: (usize, usize), color: u32) {
    for x_offset in 0..size.0 {
        for y_offset in 0..size.1 {
            let x = (pos.0 as usize).wrapping_add(x_offset);
            let y = (pos.1 as usize).wrapping_add(y_offset);
            canvas.set_pixel(x, y, color).ok();
        }
    }
}

/// Draw a line from A to B.
pub fn draw_line(canvas: &mut Canvas, a: ScreenPosition, b: ScreenPosition, color: u32) {
    if a.0 == b.0 {
        // Vertial line
        let (min, max) = if a.1 < b.1 { (a.1, b.1) } else { (b.1, a.1) };
        for y in min..=max {
            canvas.set_pixel(a.0 as usize, y as usize, color).ok();
        }
    } else {
        // Horizontal or arbitrary angle val.
        // Make sure a is < than b (x coord).
        let (a, b) = if a.0 < b.0 { (a, b) } else { (b, a) };
        let slope: f32 = ((b.1 - a.1) as f32) / ((b.0 - a.0) as f32);

        // Previous pixel y coord.
        let mut last_y: Option<isize> = None;

        for x in a.0..=b.0 {
            let off_y: f32 = slope * ((x - a.0) as f32);
            let y = a.1 + off_y as isize;

            canvas.set_pixel(x as usize, y as usize, color).ok();
            match last_y {
                Some(ly) => {
                    if ly != y {
                        if slope > 0.0f32 {
                            draw_line(
                                canvas,
                                ScreenPosition(x, ly + 1),
                                ScreenPosition(x, y),
                                color,
                            );
                        } else {
                            draw_line(
                                canvas,
                                ScreenPosition(x, ly - 1),
                                ScreenPosition(x, y),
                                color,
                            );
                        }
                    }
                }
                _ => {}
            }
            last_y = Some(y);
        }
    }
}
