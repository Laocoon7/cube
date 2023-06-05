use bevy::prelude::*;

#[derive(Component)]
pub struct Cubie {
    pub up: Option<Entity>,
    pub down: Option<Entity>,
    pub left: Option<Entity>,
    pub right: Option<Entity>,
    pub front: Option<Entity>,
    pub back: Option<Entity>,
}

impl Default for Cubie {
    fn default() -> Self {
        Self {
            up: None,
            down: None,
            left: None,
            right: None,
            front: None,
            back: None,
        }
    }
}
