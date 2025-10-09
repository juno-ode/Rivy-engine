use rivy::prelude::*;

// A simple Cube(all Cubes in world) movement system
fn player_movement(world: &mut World, input: &Input) {
    for mut cube in world.query::<&mut Cube>().iter_mut(world) {
        if input.is_down(KeyboardKey::KEY_UP) { cube.position.z -= 0.2; }
        if input.is_down(KeyboardKey::KEY_DOWN) { cube.position.z += 0.2; }
        if input.is_down(KeyboardKey::KEY_RIGHT) { cube.position.x += 0.2; }
        if input.is_down(KeyboardKey::KEY_LEFT) { cube.position.x -= 0.2; }
        if input.is_down(KeyboardKey::KEY_SPACE) { cube.position.y += 0.2; }
    }
}

fn main() {
    let mut world = World::new();    

    // Spawn a plane
    world.spawn(Plane::new(
        Vector3::zero(),
        Vector2::new(100.0, 100.0),
        Color::GRAY,
    ));

    // Spawn a cube
    world.spawn(Cube::new(
        Vector3::new(3.0, 3.0, 3.0),
        Vector3::new(2.0, 2.0, 2.0),
        Color::LIGHTGRAY,
    ));

    // Spawn the camera
    world.spawn(CAMERA::new(
        Vector3::new(0.5, 0.5, 0.5),
        Vector3::new(0.0, 0.0, 0.0),
        Vector3::new(0.0, 1.0, 0.0),
        45.0,
    ));

    // Make the app/window componits.
    let mut app = App::new(
        300,
        "Rivy(BevyECS Wrapper for raylib)",
        1920,
        1080,
        world,
        CameraMode::CAMERA_FIRST_PERSON,
    );
    
    // Add systems
    app.add_system(player_movement);
 
    // Run
    app.run();
}
