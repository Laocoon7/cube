use bevy::prelude::*;

#[derive(Resource)]
pub struct CubeColors {
    base_color: Color,
    front_color: Color,
    back_color: Color,
    left_color: Color,
    right_color: Color,
    top_color: Color,
    bottom_color: Color,
}

impl Default for CubeColors {
    fn default() -> Self {
        Self {
            base_color: Color::BLACK,
            front_color: Color::BLUE,
            back_color: Color::GREEN,
            left_color: Color::RED,
            right_color: Color::ORANGE,
            top_color: Color::WHITE,
            bottom_color: Color::YELLOW,
        }
    }
}

impl CubeColors {
    pub fn base(&self) -> Color { self.base_color }

    pub fn front(&self) -> Color { self.front_color }

    pub fn back(&self) -> Color { self.back_color }

    pub fn left(&self) -> Color { self.left_color }

    pub fn right(&self) -> Color { self.right_color }

    pub fn top(&self) -> Color { self.top_color }

    pub fn bottom(&self) -> Color { self.bottom_color }
}
