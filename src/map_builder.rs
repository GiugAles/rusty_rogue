use crate::prelude::*;
const NUM_ROOMS: usize = 20;

pub struct MapBuilder {
    pub map : Map,
    pub rooms : Vec<Rect>,
    pub player_start : Point,
}

impl MapBuilder {

    pub fn new(rng: &mut RandomNumberGenerator) -> Self {
        let mut mb = MapBuilder {
            map : Map::new(),
            rooms : Vec::new(),
            player_start : Point::zero(),
        };
        mb.fill(TileType::Wall);
        mb.build_ramdom_rooms(rng);
        /*
        mb.build_corridors = ...
        mb.player_start = ...
         */
        mb
    }

    fn fill(&mut self, tile: TileType) {
        self.map.tiles.iter_mut().for_each(|t| *t = tile);
    }

    fn build_ramdom_rooms(&mut self, rng: &mut RandomNumberGenerator) {
        while self.rooms.len() < NUM_ROOMS {
            let room = Rect::with_size(
                rng.range(1, SCREEN_WIDTH - 10),
                rng.range(1, SCREEN_HEIGHT - 10),
                rng.range(2, 10),
                rng.range(2, 10),
            );
            let mut overlap = false;
            for r in self.rooms.iter(){
                if r.intersect(&room){
                    overlap = true;
                }
            }
            if !overlap {
                room.for_each(|p| {
                    if p.x > 0 && p.x < SCREEN_WIDTH && p.y > 0 && p.y < SCREEN_HEIGHT {
                        let idx = map_idx(p.x, p.y);
                        self.map.tiles[idx] = TileType::Floor;
                    }
                });
                self.rooms.push(room)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use rand::Rng;
    use super::*;

    #[test]
    fn test_fill() {
        let mut test_map_builder = MapBuilder::new(&mut RandomNumberGenerator::new());
        test_map_builder.fill(TileType::Wall);
        test_map_builder.map.tiles.iter().for_each(|t| assert_eq!(*t, TileType::Wall));

        test_map_builder.fill(TileType::Floor);
        test_map_builder.map.tiles.iter().for_each(|t| assert_eq!(*t, TileType::Floor));
    }
}
