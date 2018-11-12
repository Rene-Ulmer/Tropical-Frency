use alloc::boxed::Box;
use camera;
use collision::Collidable;
use common::AbsolutePosition;
use common::MapPosition;
use entity::Entity;
use gfx;
use gfx::Canvas;
use gfx::Drawable;
use level;
use math;
use particle;
use projectile::Projectile;
use random;
use resources;
use {InputState, HEIGHT, WIDTH};

pub const MAP_SIZE: usize = 30;
pub const TILE_WIDTH: isize = 16;
pub const TILE_HEIGHT: isize = 16;

#[derive(Copy, Clone)]
pub enum MapTile {
    Wall(AbsolutePosition),
    Floor(AbsolutePosition),
    Door(AbsolutePosition),
    UnlockedDoor(AbsolutePosition),
    Void,
}

impl MapTile {
    pub fn is_solid(&self) -> bool {
        match &self {
            MapTile::Door(_) | MapTile::Wall(_) | MapTile::Void => true,
            MapTile::Floor(_) | MapTile::UnlockedDoor(_) => false,
        }
    }
}

impl Entity for MapTile {
    fn update(&mut self, _input_state: &InputState, _delta: f32, _level: &level::Level) {}

    fn draw(&self, canvas: &mut Canvas, camera: &camera::Camera) {
        let tileset_data = gfx::RGBData::new(resources::TILESET, resources::TILESET_SIZE);
        let tileset = gfx::Tileset::new(&tileset_data, (16, 16));

        let (idx, pos) = match &self {
            MapTile::Wall(ref pos) => ((0, 0), pos),
            MapTile::Floor(ref pos) => ((7, 1), pos),
            MapTile::Door(ref pos) => ((5, 1), pos),
            MapTile::UnlockedDoor(ref pos) => ((6, 1), pos),
            MapTile::Void => {
                return;
            }
        };
        let img = tileset.get_tile(idx);
        img.unwrap().draw(pos.to_screen(camera), canvas);
    }

    fn on_hit(&mut self, projectile: &Projectile, level: &level::Level) {
        match self {
            MapTile::Wall(_) | MapTile::Door(_) => particle::spawn_radial_particles(
                level,
                projectile.position(),
                0xFFFFFF,
                10, // count
                5,  // min_speed
                50, // max_speed
            ),
            MapTile::Floor(_) | MapTile::UnlockedDoor(_) | MapTile::Void => {}
        }
    }
}

impl Collidable for MapTile {
    fn hitbox(&self) -> Option<math::Rect> {
        match &self {
            MapTile::Wall(position) | MapTile::Door(position) => Some(math::Rect {
                pos: (position.0, position.1),
                size: (MAP_SIZE as f32, MAP_SIZE as f32),
            }),
            MapTile::Floor(_) | MapTile::UnlockedDoor(_) | MapTile::Void => None,
        }
    }
}

pub struct Map {
    tiles: Box<[Option<MapTile>; MAP_SIZE * MAP_SIZE]>,
}

impl Map {
    pub fn new() -> Self {
        let mut tiles: Box<[Option<MapTile>; MAP_SIZE * MAP_SIZE]> =
            Box::new([None; MAP_SIZE * MAP_SIZE]);
        for y in 0..MAP_SIZE {
            for x in 0..MAP_SIZE {
                let tx = x as f32 * TILE_WIDTH as f32;
                let ty = y as f32 * TILE_HEIGHT as f32;
                if x == 0 || y == 0 || x == MAP_SIZE - 1 || y == MAP_SIZE - 1 {
                    tiles[x + y * MAP_SIZE] = Some(MapTile::Wall(AbsolutePosition(tx, ty)));
                } else {
                    tiles[x + y * MAP_SIZE] = Some(MapTile::Floor(AbsolutePosition(tx, ty)));
                }
            }
        }

        // Exit will always be in the top left.
        tiles[1 + 1 * MAP_SIZE] = Some(MapTile::Door(AbsolutePosition(
            TILE_WIDTH as f32,
            TILE_HEIGHT as f32,
        )));

        // Draw some random walls.
        for _ in 0..(random::rand() % 3 + 3) {
            'next_try: loop {
                // -6 so that we always have 2 blocks free (1 for the wall, 2 free, both sides).
                let wall_length = random::rand_between(3, MAP_SIZE as u32 - 6);

                // Generate start and stop for the line. (x or y coord, decided later).
                let p1 = random::rand_between(3, MAP_SIZE as u32 - wall_length - 3);
                let p2 = p1 + wall_length;

                // Generate last part, y or x coord.
                let p3 = random::rand_between(3, MAP_SIZE as u32 - 6);

                // Decide whether line is horizontal or vertical.
                let (x1, x2, y1, y2) = if random::rand() % 2 == 0 {
                    (p1, p2, p3, p3)
                } else {
                    (p3, p3, p1, p2)
                };

                // Don't mutate the current map but operate on a copy so that we
                // can revert things back.
                let mut tiles_star = tiles.clone();

                // Looks like making a square but this really is just a line.
                for x in x1..=x2 {
                    for y in y1..=y2 {
                        let tx = x as f32 * TILE_WIDTH as f32;
                        let ty = y as f32 * TILE_HEIGHT as f32;

                        // Make sure that we do not override any other wall / door thingies.
                        match tiles_star[x as usize + y as usize * MAP_SIZE] {
                            Some(MapTile::Wall(_))
                            | Some(MapTile::Door(_))
                            | Some(MapTile::UnlockedDoor(_)) => continue 'next_try,
                            _ => {}
                        }
                        tiles_star[x as usize + y as usize * MAP_SIZE] =
                            Some(MapTile::Wall(AbsolutePosition(tx, ty)));
                    }
                }

                // Make sure that we didn't create rooms of width=1 or height=1.
                for x in 2..(MAP_SIZE - 2) {
                    for y in 2..(MAP_SIZE - 2) {
                        // Count the amount of free neighbors each field has.
                        // We don't allow less than 3 - yes this could be a
                        // corner but that's ok for now.

                        // Skip this one if it is solid.
                        if let Some(ref t) = tiles_star[x + y * MAP_SIZE] {
                            if t.is_solid() {
                                continue;
                            }
                        } else {
                            // Not present, should not happen, but whatever.
                            continue;
                        }

                        let mut num_free_neighbors = 0;
                        for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)].iter() {
                            let off_x = (x as isize + *dx) as usize;
                            let off_y = (y as isize + *dy) as usize;
                            let tile = &tiles_star[off_x + off_y * MAP_SIZE];
                            if let Some(ref t) = tile {
                                if !t.is_solid() {
                                    num_free_neighbors += 1;
                                }
                            }
                        }

                        // Not really correct, but k I guess?
                        if num_free_neighbors < 3 {
                            continue 'next_try;
                        }
                    }
                }

                tiles = tiles_star;
                break;
            }
        }

        Self { tiles }
    }

    pub fn unlock_doors(&mut self) {
        for y in 0..MAP_SIZE {
            for x in 0..MAP_SIZE {
                match self.tiles[x + y * MAP_SIZE].clone() {
                    Some(MapTile::Door(ref pos)) => {
                        self.tiles[x + y * MAP_SIZE] = Some(MapTile::UnlockedDoor(*pos));
                    }
                    _ => {}
                }
            }
        }
    }

    pub fn draw(&self, canvas: &mut Canvas, camera: &camera::Camera) {
        // Only draw the part of the map that is on screen.
        let cam = camera.get().to_map();
        let horizontal_tiles = math::floor(WIDTH as f32 / TILE_WIDTH as f32) as isize + 2;
        let vertical_tiles = math::floor(HEIGHT as f32 / TILE_HEIGHT as f32) as isize + 2;

        for x in 0..horizontal_tiles {
            for y in 0..vertical_tiles {
                if let Some(tile) = self.get_tile_at(&cam.add(&MapPosition(x, y))) {
                    tile.draw(canvas, camera);
                }
            }
        }
    }

    pub fn width(&self) -> usize {
        MAP_SIZE
    }

    pub fn height(&self) -> usize {
        MAP_SIZE
    }

    pub fn get_tile_at(&self, pos: &MapPosition) -> Option<&MapTile> {
        if (pos.0 as usize) < self.width() && (pos.1 as usize) < self.height() {
            if let Some(ref obj) = self.tiles[pos.0 as usize + (pos.1 as usize) * self.width()] {
                Some(obj)
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn get_tile_at_mut(&mut self, pos: &MapPosition) -> Option<&mut MapTile> {
        if (pos.0 as usize) < self.width() && (pos.1 as usize) < self.height() {
            if let Some(ref mut obj) = self.tiles[pos.0 as usize + (pos.1 as usize) * self.width()]
            {
                Some(obj)
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn is_solid(&self, pos: &MapPosition) -> bool {
        match self.get_tile_at(pos) {
            Some(tile) => tile.is_solid(),
            _ => false,
        }
    }
}
