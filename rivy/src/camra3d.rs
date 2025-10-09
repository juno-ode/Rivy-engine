use bevy_ecs::prelude::*;
use raylib::prelude::*;

#[derive(Component)]
pub struct CAMERA {
    pub camera3d: Camera3D, // the actual Raylib camera
}

impl CAMERA {
    pub fn new(position: Vector3, target: Vector3, up: Vector3, fovy: f32) -> Self {
        Self {
            camera3d: Camera3D::perspective(position, target, up, fovy),
        }
    }
}




