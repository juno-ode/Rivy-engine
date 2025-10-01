use raylib::prelude::*;
use bevy_ecs::{prelude::*, world};
use crate::camra3d::CommandedCamera;
use crate::darw3d::render_all;
use crate::prelude::Input;
use raylib::ffi;

// in app.rs
pub fn enable_backface_culling() {
    unsafe { ffi::rlEnableBackfaceCulling() };
}

pub fn disable_backface_culling() {
    unsafe { ffi::rlDisableBackfaceCulling() };
}
pub struct App {
    pub rl: RaylibHandle,
    pub thread: RaylibThread,
    pub world: World,
    pub camera: CommandedCamera,
    pub cam_mode: CameraMode,
    pub input: Input,
    pub systems: Vec<Box<dyn FnMut(&mut World, &Input)>>,
}

impl App {
    pub fn new(
        fps: u32,
        title: &str,
        width: i32,
        height: i32,
        world: World,
        cam: CommandedCamera,
        cam_mode: CameraMode,
    ) -> Self {
        let (mut rl, thread) = raylib::init()
            .size(width, height)
            .title(title)
            .build();

        rl.set_target_fps(fps);
        enable_backface_culling();
        rl.hide_cursor();
        Self {
            rl,
            thread,
            world,
            camera: cam,
            cam_mode,
            input: Input::new(),
            systems: Vec::new(),
        }
    }

    /// Add a system (your game logic function)
    pub fn add_system<F>(&mut self, f: F)
    where
        F: FnMut(&mut World, &Input) + 'static,
    {
        self.systems.push(Box::new(f));
    }

    /// Run the engine
    pub fn run(&mut self) {
        while !self.rl.window_should_close() {
            // update input
            self.input.update(&self.rl);

            // run each game system
            for system in &mut self.systems {
                system(&mut self.world, &self.input);
            }

            // update camera
            self.rl.update_camera(&mut self.camera.camera, self.cam_mode);

            // draw
            let mut d = self.rl.begin_drawing(&self.thread);
            d.clear_background(Color::BLACK);

            {
                let mut d3 = d.begin_mode3D(&self.camera.camera);
                render_all(&mut self.world, &mut d3);
            }

            d.draw_fps(10, 10);
        }
    }
}

