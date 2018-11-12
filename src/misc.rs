use collision;
use common;
use map;
use math;

pub fn inside_vision(
    from: &common::AbsolutePosition,
    to: &common::AbsolutePosition,
    map: &map::Map,
) -> bool {
    const STEP_COUNT: usize = 100;
    for x in 0..STEP_COUNT {
        let p = from.add(to.sub(*from).mul(x as f32 / STEP_COUNT as f32));
        let hitbox = math::Rect {
            pos: p.to_tuple(),
            size: (1f32, 1f32),
        };
        if collision::hitbox_collides_with_map(hitbox, map) {
            return false;
        }
    }
    return true;
}
