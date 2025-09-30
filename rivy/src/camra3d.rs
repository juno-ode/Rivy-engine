 // your engine wrapper
use bevy_ecs::prelude::*;
use raylib::prelude::*; // raylib types

/// Our custom camera component
#[derive(Component)]
pub struct CommandedCamera {
    pub camera: Camera3D,
}

impl CommandedCamera {
    pub fn new(position: Vector3, target: Vector3, up: Vector3, fovy: f32) -> Self {
        CommandedCamera {
            camera: Camera3D::perspective(position, target, up, fovy),
        }
    }
}

