use raylib::prelude::*;
use bevy_ecs::prelude::*;
use crate::camra3d::CommandedCamera;
use crate::darw3d::render_all;

use raylib::ffi;

/// Enable OpenGL backface culling
pub fn enable_backface_culling() {
    unsafe { ffi::rlEnableBackfaceCulling() };
}



/// Disable OpenGL backface culling
pub fn disable_backface_culling() {
    unsafe { ffi::rlDisableBackfaceCulling() };
}

pub fn initwindow(
    fps: u32,
    name: &str,
    w: i32,
    h: i32,
    world: &mut World,
    cam: &mut CommandedCamera,
    cam_mode: CameraMode,   // <--- pass in mode
) {
    let (mut rl, thread) = raylib::init()
        .size(w, h)
        .title(&name)
        .build();
    
    rl.set_target_fps(fps);
    
    while !rl.window_should_close() {
        rl.update_camera(&mut cam.camera, cam_mode); // use mode here
        
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::WHITE);
        
        {
            let mut d3 = d.begin_mode3D(&cam.camera);
            enable_backface_culling();
            render_all(world, &mut d3);
            disable_backface_culling();
        }

        d.draw_fps(10, 10);
    }
}



        