use bevy::prelude::*;

#[derive(States, Default, Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum Gamestate {
    #[default]
    Playing,
}
