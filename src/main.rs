extern crate piston_window;
extern crate opengl_graphics;
extern crate rand;

use piston_window::{
    Button,
    EventLoop,
    Input,
    Motion,
    OpenGL,
    PistonWindow,
    WindowSettings
};
use opengl_graphics::GlGraphics;

fn main() {
    const WIDTH: u32 = 1024;
    const HEIGHT: u32 = 640;

    let opengl = OpenGL::V3_2;
    let mut window: PistonWindow = WindowSettings::new(
            "Disasteroids",
            [WIDTH, HEIGHT]
        )
        .opengl(opengl)
        .samples(8)
        .exit_on_esc(true)
        .build()
        .unwrap();

    window.set_ups(60);
    window.set_max_fps(60);    

    while let Some(e) = window.next() {
        match e {
            _ => {}
        }
    }
}
