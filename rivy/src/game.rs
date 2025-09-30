

use raylib::prelude::*;

struct Enemy {
    position: Vector3,
    _health: i32,
}

pub fn game_Default() {
    let (mut rl, thread) = raylib::init()
        .size(800, 600)
        .title("Simple 3D Example")
        .build();
    
    rl.set_target_fps(60);
    rl.hide_cursor();

    let mut camera = Camera3D::perspective(
        Vector3::new(10.0, 0.0, 10.0),
        Vector3::zero(),
        Vector3::up(),
        60.0,
    );

    let enemy = Enemy {
        position: Vector3::zero(),
        _health: 100,
    };

    while !rl.window_should_close() {
        // Update
        rl.update_camera(&mut camera, CameraMode::CAMERA_FREE);

        // Draw
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::WHITE);

        {
            let mut d3 = d.begin_mode3D(&camera);

            d3.draw_cube(enemy.position, 2.0, 5.0, 2.0, Color::GRAY);
            d3.draw_cube_wires(enemy.position, 2.0, 5.0, 2.0, Color::BLACK);
            d3.draw_plane(Vector3::zero(), Vector2::new(120.0, 120.0), Color::LIGHTGRAY);
            // d3.draw_grid(10, 1.0); // uncomment if you want grid
        } // d3 dropped here

        d.draw_fps(10, 10);
    }
}


