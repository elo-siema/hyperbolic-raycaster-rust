extern crate sdl2;

use std::{cell::RefCell, rc::Rc, sync::Arc};

use sdl2::{pixels::Color, render::WindowCanvas, video::Window};

use super::canvas::Canvas;

//use crate::window::canvas::Canvas;

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

        /*canvas.set_draw_color(Color::RGB(255, 0, 0));
        canvas.clear();
        canvas.present();*/

        //canvas.borrow_mut().set_draw_color(Color::RGB(255, 0, 0));
        //canvas.borrow_mut().clear();
        //canvas.borrow_mut().present();

        View { canvas }
    }

    /// Creates a new canvas for draing a single frame and passes it to the given block. After the block has been finished, the canvas is drawn to the
    pub fn draw_canvas<F>(&mut self, drawing_function: F)
    where
        F: FnOnce(&mut Canvas),
    {
        /*let texture_creator = self.canvas.texture_creator();
        let width = self.canvas.window().size().0;
        let height = self.canvas.window().size().1;

        let mut texture = texture_creator
            .create_texture_streaming(sdl2::pixels::PixelFormatEnum::RGB24, width, height)
            .unwrap();

        texture
            .with_lock(None, |buffer: &mut [u8], pitch: usize| {
                drawing_function(&mut Canvas::new(
                    buffer,
                    pitch,
                    width as usize,
                    height as usize,
                ));
            })
            .unwrap();

        self.canvas.copy(&texture, None, None).unwrap();
        self.canvas.present();*/
        println!("dooopa");
        /*
        canvas.set_draw_color(Color::RGB(255, 0, 0));
        canvas.clear();
        canvas.present();
        */
    }
}
