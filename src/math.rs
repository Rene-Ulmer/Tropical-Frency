// We probably want to import the math functions from JS.
// However, this is *SLOW*, so we shouldn't do it too often.
// Somehow there were issues with naming them sin and cos, so
// I just added a 's' at the end. Funny, huh?
extern "C" {
    // atan2
    fn a(y: f32, x: f32) -> f32;
    // cos
    fn c(v: f32) -> f32;
    // sqrt
    fn q(v: f32) -> f32;
}

// Maybe adding a cache will make sense at some point.
// Or a precalculated lookup table?
pub fn cos(v: f32) -> f32 {
    unsafe { c(v) }
}

pub fn sin(v: f32) -> f32 {
    cos(v - PI / 2f32)
}

pub fn atan2(y: f32, x: f32) -> f32 {
    unsafe { a(y, x) }
}

pub fn sqrt(v: f32) -> f32 {
    unsafe { q(v) }
}

pub const PI: f32 = 3.1415;

pub fn deg_to_rad(deg: f32) -> f32 {
    deg * 2f32 * PI / 360f32
}

pub fn rad_to_deg(rad: f32) -> f32 {
    rad * 360f32 / (2f32 * PI)
}

pub fn round(a: f32) -> f32 {
    let delta = a - (a as isize as f32);
    if delta >= 0.5f32 {
        a as isize as f32 + 1f32
    } else {
        a as isize as f32
    }
}

pub fn floor(a: f32) -> f32 {
    a as isize as f32
}

pub fn len_vec2(val: (f32, f32)) -> f32 {
    sqrt(val.0 * val.0 + val.1 * val.1)
}

pub fn normalize(val: (f32, f32)) -> (f32, f32) {
    let len = len_vec2(val);
    (val.0 / len, val.1 / len)
}

pub fn mul_vec2(val: (f32, f32), v: f32) -> (f32, f32) {
    (val.0 * v, val.1 * v)
}

pub fn add_vec2(a: (f32, f32), b: (f32, f32)) -> (f32, f32) {
    (a.0 + b.0, a.1 + b.1)
}

pub fn sub_vec2(a: (f32, f32), b: (f32, f32)) -> (f32, f32) {
    (a.0 - b.0, a.1 - b.1)
}

pub fn scale_vec2(v: (f32, f32), len: f32) -> (f32, f32) {
    mul_vec2(normalize(v), len)
}

#[derive(Copy, Clone)]
pub struct Rect {
    pub pos: (f32, f32),
    pub size: (f32, f32),
}

impl Rect {
    pub fn contains(&self, point: (f32, f32)) -> bool {
        point.0 >= self.pos.0
            && point.0 < self.pos.0 + self.size.0
            && point.1 >= self.pos.1
            && point.1 < self.pos.1 + self.size.1
    }

    pub fn overlaps(&self, other: &Rect) -> bool {
        let self_right = self.pos.0 + self.size.0;
        let self_bot = self.pos.1 + self.size.1;

        let other_right = other.pos.0 + other.size.0;
        let other_bot = other.pos.1 + other.size.1;

        self.pos.0 < other_right
            && self_right > other.pos.0
            && self.pos.1 < other_bot
            && self_bot > other.pos.1
    }
}
