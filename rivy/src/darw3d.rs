use bevy_ecs::prelude::*;
use raylib::prelude::*;

// NOTE: The `Vector3`, `Vector2`, and `Color` types are provided by the `raylib::prelude::*` import.

/// === Components ===

/// Component representing a 3D Cube entity.
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

/// Component representing a 3D Plane entity.
#[derive(Component)]
pub struct Plane {
    pub position: Vector3,
    pub size: Vector2, // Raylib's DrawPlane uses a Vector2 for size (width, length)
    pub color: Color,
}

impl Plane {
    pub fn new(position: Vector3, size: Vector2, color: Color) -> Self {
        Self { position, size, color }
    }
}

/// === Rendering System (darw3d.rs equivalent) ===

/// Renders all Cube and Plane entities currently in the World using Raylib's 3D drawing mode.
///
/// This function uses Bevy's ECS queries to iterate over components efficiently.
///
/// NOTE: For high performance with many objects, consider moving to Instancing or batch rendering
/// instead of the current immediate-mode Raylib draw calls.
pub fn render_all(world: &mut World, d3: &mut RaylibMode3D<RaylibDrawHandle>) {
    // We access the world once to get an iterator over all entities with a Cube component.
    // The query automatically filters for entities that match the component access.
    
    // Draw cubes
    for cube in world.query::<&Cube>().iter(world) {
        // Draw the solid cube
        d3.draw_cube(
            cube.position,
            cube.size.x, cube.size.y, cube.size.z,
            cube.color
        );
        // Draw the cube wires (extra draw call, consider removing for high performance)
        d3.draw_cube_wires(
            cube.position,
            cube.size.x, cube.size.y, cube.size.z,
            Color::BLACK
        );
    }

    // Draw planes
    for plane in world.query::<&Plane>().iter(world) {
        // Raylib's DrawPlane uses Vector3 for position (center) and Vector2 for size
        d3.draw_plane(plane.position, plane.size, plane.color);
    }
}