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
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Action {
    MoveUp = 0,
    MoveDown = 1,
    MoveLeft = 2,
    MoveRight = 3,
}


#[wasm_bindgen]
pub struct Game {
    tiles: Vec<Cell>,
    map: Map,
    player_pos: Position,
}

#[wasm_bindgen]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Position {
    x: usize,
    y: usize,
}


#[wasm_bindgen]
impl Game {
    
    pub fn new(width: u32, height: u32, seed: u32) -> Game {
        let mut rng = StdRng::seed_from_u64(seed as u64);
        let map = MapBuilder::new(width as usize, height as usize)
            .with(SimpleRooms::new())
            .with(NearestCorridors::new())
            .with(AreaStartingPosition::new(XStart::LEFT, YStart::TOP))
            .with(DistantExit::new())
            .build_with_rng(&mut rng);
        let tiles = (0..map.tiles.len())
            .map(|i| if map.tiles[i] == TileType::Floor {Cell::Floor} else {Cell::Wall})
            .collect();
        let p = map.starting_point.unwrap_or(Point::new(0, 0));
        Game { tiles, map, player_pos: Position::from_point(&p) }
    }

    pub fn width(&self) -> u32 {
        self.map.width as u32
    }

    pub fn height(&self) -> u32 {
        self.map.height as u32
    }

    pub fn tiles(&self) -> *const Cell {
        self.tiles.as_ptr()
    }

    pub fn player_pos(&self) -> Position {
        self.player_pos
    }

    pub fn exit_pos(&self) -> Position {
        let p = self.map.exit_point.unwrap_or(Point::new(0, 0));
        Position { x: p.x, y: p.y }
    }

    pub fn tick(&self) {
        // Nothing yet
    }

    pub fn execute_action(&mut self, action: Action) {
        match action {
            Action::MoveUp => {
                if self.map.at(self.player_pos.x, self.player_pos.y-1) == TileType::Floor {
                    self.player_pos = Position::new(self.player_pos.x, self.player_pos.y-1)
                }
            },
            Action::MoveDown => {
                if self.map.at(self.player_pos.x, self.player_pos.y+1) == TileType::Floor {
                    self.player_pos = Position::new(self.player_pos.x, self.player_pos.y+1)
                }
            },
            Action::MoveLeft => {
                if self.map.at(self.player_pos.x-1, self.player_pos.y) == TileType::Floor {
                    self.player_pos = Position::new(self.player_pos.x-1, self.player_pos.y)
                }
            },
            Action::MoveRight => {
                if self.map.at(self.player_pos.x+1, self.player_pos.y) == TileType::Floor {
                    self.player_pos = Position::new(self.player_pos.x+1, self.player_pos.y)
                }
            },
        }
    }
}

#[wasm_bindgen]
impl Position {
    pub fn new(x: usize, y: usize) -> Position {
        Position { x, y }
    }

    pub fn col(&self) -> usize {
        self.x
    }
    
    pub fn row(&self) -> usize {
        self.y
    }
}

impl Position {
    pub fn from_point(p: &Point) -> Position {
        Position::new(p.x, p.y)
    }
}