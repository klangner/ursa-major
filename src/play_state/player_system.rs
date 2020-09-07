use std::time::SystemTime;
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Join, Read, ReadExpect, ReadStorage, System, SystemData, WriteExpect, WriteStorage};
use amethyst::input::{InputHandler, StringBindings};

use mapgen::dungeon::map::{Map, TileType};
use super::components::{Position, Player};
use super::RunState;



#[derive(SystemDesc)]
pub struct PlayerSystem {
    last_time: SystemTime,
}

impl PlayerSystem {
    pub fn new() -> PlayerSystem {
        PlayerSystem {last_time: SystemTime::now()}
    }
}

impl<'s> System<'s> for PlayerSystem {
    type SystemData = (
        ReadExpect<'s, Map>,
        WriteExpect<'s, RunState>,
        WriteStorage<'s, Position>,
        ReadStorage<'s, Player>,
        Read<'s, InputHandler<StringBindings>>,
    );

    fn run(&mut self, (map, mut state, mut positions, players, input): Self::SystemData) {
        let dt = SystemTime::now().duration_since(self.last_time).expect("Time problem?");
        if dt.as_millis() < 200 || *state != RunState::PlayerTurn {return;}

        for (_player, position) in (&players, &mut positions).join() {
            let dx = input.axis_value("player_x").unwrap_or(0.0) as i32;
            let dy = -input.axis_value("player_y").unwrap_or(0.0) as i32;
            if dx != 0 || dy != 0 {
                let new_x = position.x + dx;
                let new_y = position.y + dy;
                let tile_type = map.at(new_x as usize, new_y as usize);
                if tile_type == TileType::Floor {
                    position.translate(dx, dy);
                    *state = RunState::MonsterTurn;
                    self.last_time = SystemTime::now();
                }
            }
        }
    }
}