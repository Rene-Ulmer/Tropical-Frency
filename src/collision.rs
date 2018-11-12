use common;
use map;
use math::Rect;

pub trait Collidable {
    fn hitbox(&self) -> Option<Rect>;
}

pub fn collides<A: Collidable + ?Sized, B: Collidable + ?Sized>(a: &A, b: &B) -> bool {
    if let Some(a) = a.hitbox() {
        if let Some(b) = b.hitbox() {
            a.overlaps(&b)
        } else {
            false
        }
    } else {
        false
    }
}

pub fn hitbox_collides_with_map(hitbox: Rect, map: &map::Map) -> bool {
    // Get the tiles that might collide with this.
    const COLLISION_TILE_OFFSETS: [common::MapPosition; 4] = [
        common::MapPosition(1, 1),
        common::MapPosition(1, 0),
        common::MapPosition(0, 1),
        common::MapPosition(0, 0),
    ];

    let pos = hitbox.pos;
    let map_pos = common::AbsolutePosition(pos.0, pos.1).to_map();

    for offset in COLLISION_TILE_OFFSETS.iter() {
        if let Some(tile) = map.get_tile_at(&map_pos.add(offset)) {
            if collides(&hitbox, &*tile) {
                return true;
            }
        }
    }
    false
}

pub fn collides_with_map<T: Collidable + ?Sized>(obj: &T, map: &map::Map) -> bool {
    if let Some(hitbox) = obj.hitbox() {
        hitbox_collides_with_map(hitbox, map)
    } else {
        false
    }
}

// Can't use std::cmp::PartialOrd to make this generic :/
fn is_in_interval(a: f32, b: f32, w: f32) -> bool {
    a <= b && a + w > b
}

impl Collidable for Rect {
    fn hitbox(&self) -> Option<Rect> {
        Some(self.clone())
    }
}
