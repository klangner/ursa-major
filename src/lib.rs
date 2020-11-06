use wasm_bindgen::prelude::*;
use rand::prelude::*;
use mapgen::{Map, MapBuilder, TileType, geometry::Point};
use mapgen::filter::*;


#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Floor = 0,
    Wall = 1,
}

#[wasm_bindgen]
pub struct World {
    width: u32,
    height: u32,
    tiles: Vec<Cell>,
    map: Map,
}

#[wasm_bindgen]
pub struct Position {
    col: usize,
    row: usize,
}


#[wasm_bindgen]
impl World {
    
    fn new(width: u32, height: u32, map: Map) -> World {
        let tiles = (0..map.tiles.len())
            .map(|i| if map.tiles[i] == TileType::Floor {Cell::Floor} else {Cell::Wall})
            .collect();
        World { width, height, tiles, map }
    }

    pub fn new_simple_rooms(width: u32, height: u32, seed: u32) -> World {
        let mut rng = StdRng::seed_from_u64(seed as u64);
        let map = MapBuilder::new(width as usize, height as usize)
            .with(SimpleRooms::new())
            .with(NearestCorridors::new())
            .with(AreaStartingPosition::new(XStart::LEFT, YStart::TOP))
            .with(DistantExit::new())
            .build_with_rng(&mut rng);
        World::new(width, height, map)
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn tiles(&self) -> *const Cell {
        self.tiles.as_ptr()
    }

    pub fn player_pos(&self) -> Position {
        let p = self.map.starting_point.unwrap_or(Point::new(0, 0));
        Position { col: p.x, row: p.y }
    }

    pub fn exit_pos(&self) -> Position {
        let p = self.map.exit_point.unwrap_or(Point::new(0, 0));
        Position { col: p.x, row: p.y }
    }

}

#[wasm_bindgen]
impl Position {
    pub fn new(col: usize, row: usize) -> Position {
        Position { col, row }
    }

    pub fn col(&self) -> usize {
        self.col
    }
    
    pub fn row(&self) -> usize {
        self.row
    }
}