use amethyst::derive::SystemDesc;
use amethyst::ecs::{Join, Read, ReadStorage, System, SystemData, WriteStorage};
use amethyst::input::{InputHandler, StringBindings};

use mapgen::dungeon::map::{Map, TileType};
use super::components::{Position, Player};



#[derive(SystemDesc)]
pub struct PlayerSystem {
    last_x: f32,
    last_y: f32
}

impl PlayerSystem {
    pub fn new() -> PlayerSystem {
        PlayerSystem{ last_x: 0.0, last_y: 0.0}
    }
}

impl<'s> System<'s> for PlayerSystem {
    type SystemData = (
        Read<'s, Map>,
        WriteStorage<'s, Position>,
        ReadStorage<'s, Player>,
        Read<'s, InputHandler<StringBindings>>,
    );

    fn run(&mut self, (map, mut positions, players, input): Self::SystemData) {
        for (_player, position) in (&players, &mut positions).join() {
            if let Some(player_x) = input.axis_value("player_x") {
                let x = position.x + player_x as i32;
                let tile_type = map.at(x as usize, position.y as usize);
                if x > 0 && x < map.width as i32 
                    && tile_type == TileType::Floor 
                    && player_x != self.last_x 
                {
                    position.x = x;
                    self.last_x = player_x;
                }
            }
            if let Some(player_y) = input.axis_value("player_y") {
                let y = position.y - player_y as i32;
                let tile_type = map.at(position.x as usize, y as usize);
                if y > 0 && y < map.height as i32 
                    && tile_type == TileType::Floor 
                    && player_y != self.last_y 
                {
                    position.y = y;
                    self.last_y = player_y;
                }
            }
        }
    }
}