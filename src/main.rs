extern crate sdl2;

#[macro_use]
extern crate lazy_static;

use std::{cell::RefCell, cell::RefMut, panic, rc::Rc};

mod emscripten;
mod game;
mod hyperbolic_renderer;
mod poncaire_renderer;
mod utils;
mod window;
use game::hypermap::HyperMap;
use game::Game;
use hyperbolic_renderer::Renderer;
use sdl2::{
    pixels::Color, render::Texture, render::TextureCreator, render::WindowCanvas,
    video::WindowContext,
};
//use poncaire_renderer::Renderer;
use std::process::exit;
use std::time::Duration;
use utils::geometry::Point;
use utils::hyperpoint::Hyperpoint;
use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::Window as z;
use web_sys::*;
use window::event::Keycode;
use window::Window;
use window::{canvas::Canvas, event::Event};

thread_local! {
    pub static WINDOW: RefCell<Window> = RefCell::new(Window::new());
    pub static RENDERER: RefCell<Renderer> = RefCell::new(Renderer::new(Game::new(HyperMap::new(include_str!("../assets/5square.json"))), 1.0, 0.75, 1., 0.25));
    pub static INITIAL_RUN: RefCell<bool> = RefCell::new(true);
}

fn box_forget<T>(item: Box<T>) {
    std::mem::forget(item);
}

fn main() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    // Initialize the graphics and event handling.
    /*
        let mut window = Box::new(Window::new());

        let map = HyperMap::new(include_str!("../assets/singlewall.json"));
        let map = HyperMap::new(include_str!("../assets/5square.json"));
        // Load the game and place the player within the map.
        let game = Game::new(map);

        // Initialize the renderer
        let mut renderer = Box::new(Renderer::new(game, 1.0, 0.75, 1., 0.25));

        // Require a screen refresh after startup.
        let mut initial_run = Box::new(true);
    */
    // Run main loop on normal UI targets.
    //std::thread::sleep_ms(1000);

    //main_loop(&mut window,/* &mut renderer,*/ &initial_run);
    //let mut canvas = window.view.canvas.borrow_mut();

    /*
        #[cfg(not(target_os = "emscripten"))]
        loop {
            let mut texture = texture_creator
            .create_texture_streaming(sdl2::pixels::PixelFormatEnum::RGB24, width, height)
            .unwrap();
            main_loop(&mut renderer, &mut canvas, texture, width, height,&initial_run);
            initial_run = false;
        }

    *//*
    let width =window.view.canvas.window().size().0;
    println!("140");
    let height = window.view.canvas.window().size().1;
    println!("142");
    println!("{}, {}", width, height);

    let texture_creator = window.view.canvas.texture_creator();
    let mut texture = match texture_creator
        .create_texture_streaming(sdl2::pixels::PixelFormatEnum::RGB24, width, height) {
            Ok(tex) => {
                println!("succ1");
                tex
            }
            Err(e) => {
                println!("errr1 {}", e.to_string());

                panic!("err");
            }
        };*/
    // Run main loop on web targets.
    /*
    let w = web_sys::window().unwrap();
    let cb = Closure::wrap(Box::new(move || {
        main_loop(&mut window, &mut renderer,  &initial_run);

    }) as Box<FnMut()>);
    unsafe {
        w.request_animation_frame( cb.as_ref().unchecked_ref());

    }*/

    #[cfg(target_os = "emscripten")]
    {
        //use emscripten::emscripten;
        emscripten::emscripten::set_main_loop_callback(|| {
            main_loop();
        });
    }
    //box_forget(window);
    //box_forget(renderer);
    //box_forget(initial_run);
    println!("Exiting main");
}

/// The main event handling loop.
fn main_loop() {
    WINDOW.with(|w| {
        let mut window = w.borrow_mut();
        INITIAL_RUN.with(|w| {
            let mut initial_run = w.borrow_mut();
            RENDERER.with(|w| {
                let mut renderer = w.borrow_mut();

                let mut needs_refresh = *initial_run;
                //println!("l00p");
                //println!("138");

                // Get pending UI events
                match window.event_source.poll_next_event() {
                    None => {}
                    Some(Event::Quit) => {
                        exit(0);
                    }
                    Some(Event::Resize) => {
                        needs_refresh = true;
                    }
                }

                // Handle key presses
                for keycode in window.event_source.pressed_keycodes() {
                    needs_refresh = true;
                    let movement_speed = 0.01;
                    let rotation_speed = 0.05;

                    match keycode {
                        // Arrow up/down: Move player forward/backwards
                        Keycode::Up => renderer.game.move_player(-movement_speed),
                        Keycode::Down => renderer.game.move_player(movement_speed),

                        // AWSD: Move player forward/backwards and strafe left/right
                        Keycode::W => renderer.game.move_player(-movement_speed),
                        Keycode::S => renderer.game.move_player(movement_speed),
                        Keycode::D => renderer.game.strafe_player(movement_speed),
                        Keycode::A => renderer.game.strafe_player(-movement_speed),

                        // Arrow right/left: Rotate player
                        Keycode::Right => {
                            renderer.game.rotate_player(-rotation_speed);
                        }
                        Keycode::Left => {
                            renderer.game.rotate_player(rotation_speed);
                        }
                    }
                }
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
                //renderer.game.move_player(0.01);
                //println!("dooopa");
                //println!("{}",Arc::strong_count(canvas.canvas));
                //rintln!("131");

                //canvas.set_draw_color(Color::RGB(255, 0, 0));
                //canvas.clear();
                //let mut my_canvas = window::canvas::Canvas::new()

                //println!("147");

                //println!("goodbye");
                // Refresh screen if needed
                if needs_refresh {
                    let width = window.view.canvas.window().size().0;
                    //println!("140");
                    let height = window.view.canvas.window().size().1;
                    //println!("142");
                    //println!("{}, {}", width, height);

                    let texture_creator = window.view.canvas.texture_creator();
                    let mut texture = match texture_creator.create_texture_streaming(
                        sdl2::pixels::PixelFormatEnum::RGB24,
                        width,
                        height,
                    ) {
                        Ok(tex) => {
                            //println!("succ2");
                            tex
                        }
                        Err(e) => {
                            //println!("errr2 {}", e.to_string());

                            panic!("err");
                        }
                    };
                    texture
                        .with_lock(None, |buffer: &mut [u8], pitch: usize| {
                            renderer.render(&mut Canvas::new(
                                buffer,
                                pitch,
                                width as usize,
                                height as usize,
                            ));
                        })
                        .unwrap();
                    match window.view.canvas.copy(&texture, None, None) {
                        Ok(tex) => {
                            //println!("copy success");
                            tex
                        }
                        Err(e) => {
                            println!("copy err {}", e.to_string());

                            panic!("err");
                        }
                    };
                    //println!("155");

                    window.view.canvas.present();
                    //window.view.draw_canvas({move  |mut canvas|() /*renderer.render(&mut canvas)*/ });
                }
            });
            w.replace_with(|_| false);
        });
    });
}
