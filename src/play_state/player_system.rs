use amethyst::derive::SystemDesc;
use amethyst::ecs::{Join, Read, ReadStorage, System, SystemData, WriteStorage};
use amethyst::input::{InputHandler, StringBindings};

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
        WriteStorage<'s, Position>,
        ReadStorage<'s, Player>,
        Read<'s, InputHandler<StringBindings>>,
    );

    fn run(&mut self, (mut positions, players, input): Self::SystemData) {
        for (_player, position) in (&players, &mut positions).join() {
            if let Some(player_x) = input.axis_value("player_x") {
                if player_x != self.last_x {
                    position.x += player_x as i32;
                    self.last_x = player_x;
                }
            }
            if let Some(player_y) = input.axis_value("player_y") {
                if player_y != self.last_y {
                    position.y -= player_y as i32;
                    self.last_y = player_y;
                }
            }
        }
    }
}