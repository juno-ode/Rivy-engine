use bevy_ecs::prelude::*;
use raylib::prelude::*;

/// === Components ===

#[derive(Component)]
pub struct Cube {
    pub position: Vector3,
    pub size: Vector3,
    pub color: Color,
}

impl Cube {
    pub fn new(position: Vector3, size: Vector3, color: Color) -> Self {
        Self { position, size, color }
    }
}

#[derive(Component)]
pub struct Plane {
    pub position: Vector3,
    pub size: Vector2,
    pub color: Color,
}

impl Plane {
    pub fn new(position: Vector3, size: Vector2, color: Color) -> Self {
        Self { position, size, color }
    }
}

/// =// darw3d.rs
pub fn render_all(world: &mut World, d3: &mut RaylibMode3D<RaylibDrawHandle>) {
    // Draw cubes
    for cube in world.query::<&Cube>().iter(world) {
        d3.draw_cube(cube.position, cube.size.x, cube.size.y, cube.size.z, cube.color);
        d3.draw_cube_wires(cube.position, cube.size.x, cube.size.y, cube.size.z, Color::BLACK);
    }

    // Draw planes
    for plane in world.query::<&Plane>().iter(world) {
        d3.draw_plane(plane.position, plane.size, plane.color);
    }
}

