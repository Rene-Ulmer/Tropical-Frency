use alloc::vec::Vec;
use common::MapPosition;
use map::Map;

struct Node {
    distance: usize,
    last: Option<MapPosition>,
}

pub fn find_path(from: &MapPosition, to: &MapPosition, map: &Map) -> Option<Vec<MapPosition>> {
    if from.0 as usize >= map.width()
        || from.1 as usize >= map.height()
        || to.0 as usize >= map.width()
        || to.1 as usize >= map.height()
    {
        return None;
    }

    let mut list: Vec<MapPosition> = Vec::new();
    let mut paths: Vec<Node> = Vec::with_capacity(map.width() * map.height());

    for _ in 0..map.height() {
        for _ in 0..map.width() {
            paths.push(Node {
                distance: 0xFFFFFFFF,
                last: None,
            });
        }
    }
    // paths[x + y * width]
    paths[from.0 as usize + from.1 as usize * map.width()].distance = 0;
    list.push(*from);

    // Timeout :)
    for _ in 0..400 {
        let mut newlist: Vec<MapPosition> = Vec::new();
        for node in list.iter() {
            for offset in [
                MapPosition(0, 1),
                MapPosition(0, -1),
                MapPosition(1, 0),
                MapPosition(-1, 0),
            ]
                .iter()
            {
                let neighbour = node.add(offset);
                // Checking for wall.
                if !map.is_solid(&neighbour) {
                    let neighbour_coords =
                        neighbour.0 as usize + neighbour.1 as usize * map.width();
                    let own_coords = node.0 as usize + node.1 as usize * map.width();

                    if paths[own_coords].distance + 1 < paths[neighbour_coords].distance {
                        // Update cost & previous element.
                        paths[neighbour_coords].distance = paths[own_coords].distance + 1;
                        paths[neighbour_coords].last = Some(*node);
                        newlist.push(neighbour);
                    }
                }
            }

            if paths[to.0 as usize + to.1 as usize * map.width()]
                .last
                .is_some()
            {
                let mut path = Vec::new();
                let mut current_pos = *to;

                while paths[current_pos.0 as usize + current_pos.1 as usize * map.width()]
                    .last
                    .is_some()
                {
                    path.push(MapPosition(current_pos.0 as isize, current_pos.1 as isize));
                    current_pos = paths
                        [current_pos.0 as usize + current_pos.1 as usize * map.width()].last
                    .unwrap();
                }
                path.push(*from);

                return Some(path);
            }
        }
        list = newlist;
    }

    None
}
