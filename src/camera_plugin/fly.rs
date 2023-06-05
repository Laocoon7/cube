use bevy::{input::mouse::MouseMotion, prelude::*};

#[derive(Component)]
pub struct FlyCamera {
    pub speed: f32,
    pub rotation_speed_x: f32,
    pub rotation_speed_y: f32,
}

impl Default for FlyCamera {
    fn default() -> Self {
        Self {
            speed: 5.0,
            rotation_speed_x: 0.003,
            rotation_speed_y: 0.003,
        }
    }
}

pub fn fly_camera(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mouse_input: Res<Input<MouseButton>>,
    mut mouse_events: EventReader<MouseMotion>,
    mut q_camera: Query<(&mut Transform, &FlyCamera)>,
) {
    for (mut transform, fly_camera) in q_camera.iter_mut() {
        // Translation
        let mut translation = Vec3::ZERO;
        if keyboard_input.pressed(KeyCode::W) {
            translation += transform.forward();
        }
        if keyboard_input.pressed(KeyCode::S) {
            translation += -transform.forward();
        }
        if keyboard_input.pressed(KeyCode::A) {
            translation += -transform.right();
        }
        if keyboard_input.pressed(KeyCode::D) {
            translation += transform.right();
        }
        if keyboard_input.pressed(KeyCode::E) {
            translation += transform.up();
        }
        if keyboard_input.pressed(KeyCode::Q) {
            translation += -transform.up();
        }
        transform.translation += translation * time.delta_seconds() * fly_camera.speed;

        // Rotationw
        if mouse_input.pressed(MouseButton::Right) {
            for mouse_event in mouse_events.iter() {
                let right = transform.right();
                transform.rotate_axis(Vec3::Y, -mouse_event.delta.x * fly_camera.rotation_speed_x);
                transform.rotate_axis(right, -mouse_event.delta.y * fly_camera.rotation_speed_y);
            }
        }
    }
}
