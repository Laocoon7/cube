use bevy::prelude::*;

#[derive(Clone)]
pub struct CubeColors {
    pub base: Color,
    pub up: Color,
    pub down: Color,
    pub left: Color,
    pub right: Color,
    pub front: Color,
    pub back: Color,
}

impl Default for CubeColors {
    fn default() -> Self { Self::western_black() }
}

impl CubeColors {
    pub fn japanese_black() -> Self {
        Self {
            base: Color::BLACK,
            up: Color::WHITE,
            down: Color::YELLOW,
            left: Color::RED,
            right: Color::ORANGE,
            front: Color::GREEN,
            back: Color::BLUE,
        }
    }

    pub fn japanese_white() -> Self {
        Self {
            base: Color::WHITE,
            up: Color::BLACK,
            down: Color::YELLOW,
            left: Color::RED,
            right: Color::ORANGE,
            front: Color::GREEN,
            back: Color::BLUE,
        }
    }

    pub fn western_black() -> Self {
        Self {
            base: Color::BLACK,
            up: Color::WHITE,
            down: Color::YELLOW,
            left: Color::RED,
            right: Color::ORANGE,
            front: Color::GREEN,
            back: Color::BLUE,
        }
    }

    pub fn western_white() -> Self {
        Self {
            base: Color::WHITE,
            up: Color::BLACK,
            down: Color::YELLOW,
            left: Color::RED,
            right: Color::ORANGE,
            front: Color::GREEN,
            back: Color::BLUE,
        }
    }
}
