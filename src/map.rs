use crate::prelude::*;
const NUM_TILES: usize = (SCREEN_WIDTH * SCREEN_HEIGHT) as usize;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum TileType {
    Wall,
    Floor,
}

pub struct Map {
    pub tiles: Vec<TileType>,
}

pub  fn map_idx(x: i32, y: i32) -> usize {
    ((y * SCREEN_WIDTH) + x) as usize
}

impl Map {
    pub fn new() -> Self {
        Self {
            tiles: vec![TileType::Floor; NUM_TILES],
        }
    }

    pub fn render(&self, ctx: &mut BTerm, camera: &Camera) {
        ctx.set_active_console(0);
        for y in camera.top_y .. camera.bottom_y {
            for x in camera.left_x .. camera.right_x {
                if self.in_bounds(Point::new(x, y)) {
                    let idx = map_idx(x, y);
                    match self.tiles[idx] {
                        TileType::Floor => {
                            ctx.set(
                                x - camera.left_x,
                                y - camera.top_y,
                                WHITE,
                                BLACK,
                                to_cp437('.')
                            );
                        }
                        TileType::Wall => {
                            ctx.set(
                                x - camera.left_x,
                                y - camera.top_y,
                                WHITE,
                                BLACK,
                                to_cp437('#')
                            );
                        }
                    }
                }
            }
        }
    }

    pub fn in_bounds(&self, point : Point) -> bool {
        0 <= point.x && point.x < SCREEN_WIDTH
            && 0 <= point.y && point.y < SCREEN_HEIGHT
    }

    pub fn can_enter_tile(&self, point : Point) -> bool {
        self.in_bounds(point)
        && self.tiles[map_idx(point.x, point.y)] == TileType::Floor
    }

    pub fn try_idx(&self, point : Point) -> Option<usize> {
        if !self.in_bounds(point) {
            None
        } else {
            Some(map_idx(point.x, point.y))
        }
    }
}


#[cfg(test)]
mod tests {
    use rand::Rng;
    use super::*;

    #[test]
    fn test_default_map() {
        let default_map= Map::new();
        assert_eq!(default_map.tiles.len(), (SCREEN_WIDTH * SCREEN_HEIGHT) as usize)
    }

    #[test]
    fn test_map_idx() {
        for index in 0..NUM_TILES as i32 {
            assert_eq!(index, map_idx(index % SCREEN_WIDTH, index / SCREEN_WIDTH) as i32);
        }
    }

    #[test]
    fn test_in_bounds() {
        let default_map = Map::new();
        for y in 0..SCREEN_HEIGHT {
            for x in 0..SCREEN_WIDTH {
                assert!(default_map.in_bounds(Point{x,y}));
            }
        }
        assert!(!default_map.in_bounds(Point::new(SCREEN_WIDTH + 1, SCREEN_HEIGHT + 1 )));
        assert!(!default_map.in_bounds(Point::new(-1, -1)));
    }

    #[test]
    fn test_can_enter_tile() {
        let mut default_map = Map::new();
        let mut rng = rand::thread_rng();
        for _ in 0..50 { // abitrary number of tries
            let pos_x = rng.gen_range(0..SCREEN_WIDTH);
            let pos_y = rng.gen_range(0..SCREEN_HEIGHT);
            let idx = map_idx(pos_x, pos_y);
            // in case we have two identical rand numbers
            default_map.tiles[idx] = TileType::Floor;
            assert!(default_map.can_enter_tile(Point{x: pos_x, y: pos_y}));

            default_map.tiles[idx] = TileType::Wall;
            assert!(!default_map.can_enter_tile(Point{x: pos_x, y: pos_y}));
        }
    }

    #[test]
    fn test_try_idx() {
        let default_map = Map::new();
        for y in 0..SCREEN_HEIGHT {
            for x in 0..SCREEN_WIDTH {
                assert_eq!(default_map.try_idx(Point{x,y}), Some(map_idx(x, y)));
            }
        }
        assert_eq!(default_map.try_idx(Point::new(SCREEN_WIDTH + 1, SCREEN_HEIGHT + 1 )), None);
        assert_eq!(default_map.try_idx(Point::new(-1, -1)), None);
    }
}
