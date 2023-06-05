use bevy::prelude::*;

#[derive(Component)]
pub struct Cube {
    pub size: usize,
    pub cubies: Vec<Entity>,
}

impl Cube {
    pub fn new(size: usize) -> Self {
        Self {
            size,
            cubies: Vec::new(),
        }
    }
}
