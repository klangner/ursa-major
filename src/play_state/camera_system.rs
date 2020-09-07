use amethyst::{
    core::Transform,
    derive::SystemDesc,
    ecs::{Join, ReadExpect, System, SystemData, ReadStorage, WriteStorage},
    renderer::Camera,
};
use mapgen::dungeon::map::Map;
use super::components::{Position, Player};



#[derive(SystemDesc)]
pub struct CameraSystem {
}

impl CameraSystem {
    pub fn new() -> CameraSystem {
        CameraSystem {}
    }
}

impl<'s> System<'s> for CameraSystem {
    type SystemData = (
        ReadExpect<'s, Map>,
        ReadStorage<'s, Position>,
        ReadStorage<'s, Player>,
        ReadStorage<'s, Camera>,
        WriteStorage<'s, Transform>,
    );

    fn run(&mut self, (map, positions, players, cameras, mut transforms): Self::SystemData) {
        for (_player, position) in (&players, &positions).join() {
            for (_camera, transform) in (&cameras, &mut transforms).join() {
                let x = (position.x - (map.width as i32)/2) * 52;
                let y = (position.y - (map.height as i32)/2) * 52;
                transform.set_translation_x(x as f32);
                transform.set_translation_y(-y as f32);
            }
        }
    }
}