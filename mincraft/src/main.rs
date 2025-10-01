use rivy::prelude::*;

// A player movement system
fn player_movement(world: &mut World, input: &Input) {
    let mut query = world.query::<&mut Cube>();
    for mut cube in query.iter_mut(world) {
        if input.is_down(KeyboardKey::KEY_UP) {
            cube.position.z -= 0.2;
        }
        if input.is_down(KeyboardKey::KEY_DOWN) {
            cube.position.z += 0.2;
        }
        if input.is_down(KeyboardKey::KEY_RIGHT) {
            cube.position.x += 0.2;
        }
        if input.is_down(KeyboardKey::KEY_LEFT) {
            cube.position.x -= 0.2;
        }
    }
}

// Another example system


fn main() {
    let mut world = World::new();

    // Spawn a cube
    world.spawn(Plane::new(
    Vector3::zero(),
    Vector2::new(100.0,100.0),
    Color::GRAY
    ));
    world.spawn(Cube::new(
        Vector3::new(0.0, 0.0, 0.0),
        Vector3::new(2.0, 2.0, 2.0),
        Color::LIGHTGRAY,
    ));

    // Setup camera
    let cam = CommandedCamera::new(
        Vector3::new(5.0, 5.0, 5.0),
        Vector3::zero(),
        Vector3::up(),
        45.0,
    );

    // Make the app
    let mut app = App::new(
        60,
        "My Game",
        800,
        600,
        world,
        cam,
        CameraMode::CAMERA_FIRST_PERSON,
    );

    // Add systems (your logic)
    app.add_system(player_movement);
    

    // Run the game
    app.run();
}
