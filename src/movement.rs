use alloc::vec::Vec;
use collision::collides;
use common::{AbsolutePosition, MapPosition};
use level;
use map::{TILE_HEIGHT, TILE_WIDTH};
use math;
use misc;
use pathfinding;

pub fn move_to_player(
    hitbox: &math::Rect,
    force: &mut (f32, f32),
    max_speed: f32,
    level: &level::Level,
) -> Option<()> {
    let pos = AbsolutePosition(hitbox.pos.0, hitbox.pos.1);
    let pos2 = AbsolutePosition(hitbox.pos.0 + hitbox.size.0, hitbox.pos.1 + hitbox.size.1);
    let map = level.map.borrow();
    let path: Vec<MapPosition> = pathfinding::find_path(
        &level.player.borrow().get_center_position().to_map(),
        &pos.to_map(),
        &map,
    )?;

    let mut target_tile: &MapPosition = &pos.to_map();
    let path_length = path.len();

    const HALF_TILE: AbsolutePosition =
        AbsolutePosition(TILE_WIDTH as f32 / 2f32, TILE_HEIGHT as f32 / 2f32);

    for x in 1..path_length {
        let p = &path[path_length - x].to_absolute();
        // Check whether the center of the tile is inside the vision.
        if misc::inside_vision(&p.add(HALF_TILE), &pos, &map)
            && misc::inside_vision(&p.add(HALF_TILE), &pos2, &map)
        {
            target_tile = &path[path_length - x];
            break;
        }
    }

    let target_center = target_tile.to_absolute();
    let path = (target_center.0 - pos.0, target_center.1 - pos.1);
    let distance = math::len_vec2(path);
    let direction_vec = math::normalize(path);

    let speed = if distance > max_speed {
        max_speed
    } else {
        distance
    };

    *force = math::mul_vec2(direction_vec, speed);
    Some(())
}

/// Applies a given force to the rectangle. If it collides with something in the
/// game object, the object will stop and the force will be updated.
pub fn apply_force(
    pos: &mut (f32, f32),
    size: (f32, f32),
    force: &mut (f32, f32),
    knockback_force: &mut (f32, f32),
    delta: f32,
    level: &level::Level,
) {
    let old_pos = *pos;
    let map = level.map.borrow();

    // Perform only map tile collision checks.
    // Check the four neighbor tiles.
    const COLLISION_TILE_OFFSETS: [MapPosition; 4] = [
        MapPosition(1, 1),
        MapPosition(1, 0),
        MapPosition(0, 1),
        MapPosition(0, 0),
    ];

    // X coordinates first.
    pos.0 += (force.0 + knockback_force.0) * delta / 1000f32;

    // Grab our tile position.
    let player_tile_pos = AbsolutePosition(pos.0, pos.1).to_map();

    let hitbox = |pos: &(f32, f32)| math::Rect { pos: *pos, size };

    // Check only 4 potentially interesting tiles.
    for offset in COLLISION_TILE_OFFSETS.iter() {
        if let Some(tile) = map.get_tile_at(&player_tile_pos.add(offset)) {
            if collides(&hitbox(&pos), &*tile) {
                pos.0 = old_pos.0;
                force.0 *= 0.6f32;
                knockback_force.0 *= 0.6f32;
                break;
            }
        }
    }

    // Y coordinates
    pos.1 += (force.1 + knockback_force.1) * delta / 1000f32;
    let player_tile_pos = AbsolutePosition(pos.0, pos.1).to_map();

    for offset in COLLISION_TILE_OFFSETS.iter() {
        if let Some(tile) = map.get_tile_at(&player_tile_pos.add(offset)) {
            if collides(&hitbox(&pos), &*tile) {
                pos.1 = old_pos.1;
                force.1 *= 0.6f32;
                knockback_force.1 *= 0.6f32;
                break;
            }
        }
    }

    const SLOWDOWN_FACTOR: f32 = 1f32;
    if math::len_vec2(*knockback_force) <= SLOWDOWN_FACTOR * delta {
        *knockback_force = (0f32, 0f32);
    } else {
        let slowdown = math::scale_vec2(*knockback_force, SLOWDOWN_FACTOR * delta);

        *knockback_force = math::sub_vec2(*knockback_force, slowdown);
    }
}
