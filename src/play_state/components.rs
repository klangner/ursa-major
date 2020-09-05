use amethyst::ecs::{Component, DenseVecStorage};

pub struct Player {}

impl Component for Player {
    type Storage = DenseVecStorage<Self>;
}

pub struct Position {
    pub x: i32,
    pub y: i32
}

impl Position {
    pub fn new(x: i32, y: i32) -> Position {
        Position {x, y}
    }
}

impl Component for Position {
    type Storage = DenseVecStorage<Self>;
}

pub struct Renderable {
    pub glyph: usize
}

impl Renderable {
    pub fn new(glyph: usize) -> Renderable {
        Renderable {glyph}
    }
}

impl Component for Renderable {
    type Storage = DenseVecStorage<Self>;
}