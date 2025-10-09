
pub mod winit;
pub mod darw3d;
pub mod camra3d;  
pub mod add;
pub mod runtimesys;
 // important for Bevy-style prelude imports


pub mod prelude {
    pub use crate::camra3d::CAMERA;
    pub use crate::darw3d::{Cube, Plane, render_all};
    pub use crate::winit::App;
    pub use bevy_ecs::prelude::*;
    pub use raylib::prelude::*;
    pub use crate::runtimesys::Input;
}
