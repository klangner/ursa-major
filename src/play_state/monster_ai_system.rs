use amethyst::derive::SystemDesc;
use amethyst::ecs::{ReadExpect, System, SystemData, WriteExpect};
use mapgen::Map;
use super::RunState;



#[derive(SystemDesc)]
pub struct MonsterAISystem {
}

impl MonsterAISystem {
    pub fn new() -> MonsterAISystem {
        MonsterAISystem {}
    }
}

impl<'s> System<'s> for MonsterAISystem {
    type SystemData = (
        ReadExpect<'s, Map>,
        WriteExpect<'s, RunState>,
    );

    fn run(&mut self, (_map, mut state): Self::SystemData) {
        if *state == RunState::MonsterTurn {
            *state = RunState::PlayerTurn;
        }
    }
}