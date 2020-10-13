extern crate sdl2;

use sdl2::render::WindowCanvas;


/// A screen tile that can be used for drawing. (E.g. the content tile of a window or a HTML canvas.)
pub struct View {
    pub canvas: WindowCanvas,
}

impl View {
    /// Creates a new view using an SDL context.
    pub fn new(sdl_context: &sdl2::Sdl) -> View {
        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem
            .window("Ray Casting Demo", 800, 600)
            .position_centered()
            .opengl()
            .build()
            .map_err(|e| e.to_string())
            .unwrap();
        let canvas = window
            .into_canvas()
            .build()
            .map_err(|e| e.to_string())
            .unwrap();

        View { canvas }
    }
}
