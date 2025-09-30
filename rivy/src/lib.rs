
pub mod winit;
pub mod darw3d;
pub mod camra3d;
pub mod game;
pub mod add;
 // important for Bevy-style prelude imports


pub mod prelude {
    pub use crate::camra3d::CommandedCamera;
    pub use crate::darw3d::{Cube, Plane, render_all};
    pub use crate::winit::initwindow;
    pub use bevy_ecs::prelude::*;
    pub use raylib::prelude::*;
}