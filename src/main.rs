extern crate sdl2;

use std::{cell::RefCell, panic};

mod emscripten;
mod game;
mod hyperbolic_renderer;
mod poincare_renderer;
mod utils;
mod window;
use game::hypermap::HyperMap;
use game::Game;
use hyperbolic_renderer::Renderer;
//use poincare_renderer::Renderer;
use std::process::exit;
use window::event::Keycode;
use window::Window;
use window::{canvas::Canvas, event::Event};

// "Globals", to solve the issue of Emscripten losing reference to these
// when referenced by the closure called by emscripten main loop
// (probably related to https://github.com/Rust-SDL2/rust-sdl2/issues/884 )
thread_local! {
    pub static WINDOW: RefCell<Window> = RefCell::new(Window::new());
    pub static RENDERER: RefCell<Renderer> = RefCell::new(Renderer::new(Game::new(HyperMap::new(include_str!("../assets/5square.json"))), 1.0, 0.75, 1., 0.25));
    pub static INITIAL_RUN: RefCell<bool> = RefCell::new(true);
}

fn main() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));

    #[cfg(not(target_os = "emscripten"))]
    {
        loop {
            main_loop();
        }
    }

    #[cfg(target_os = "emscripten")]
    {
        emscripten::emscripten::set_main_loop_callback(|| {
            main_loop();
        });
    }
}

/// The main event handling loop.
fn main_loop() {
    // Needs to be staggered like this because of Emscripten crashing
    // when I do it any other way.
    WINDOW.with(|w| {
        let mut window = w.borrow_mut();
        INITIAL_RUN.with(|w| {
            let initial_run = w.borrow_mut();
            RENDERER.with(|w| {
                let mut renderer = w.borrow_mut();

                // todo:: update initial run so it does not break Emscripten
                // Right now it redraws every frame
                let mut needs_refresh = *initial_run; 

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
                    let rotation_speed = 0.01;

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

                // Refresh screen if needed
                if needs_refresh {
                    let width = window.view.canvas.window().size().0;
                    let height = window.view.canvas.window().size().1;

                    let texture_creator = window.view.canvas.texture_creator();
                    let mut texture = match texture_creator.create_texture_streaming(
                        sdl2::pixels::PixelFormatEnum::RGB24,
                        width,
                        height,
                    ) {
                        Ok(tex) => {
                            tex
                        }
                        Err(e) => {
                            println!("Error creating texture! {}", e.to_string());
                            panic!("Error creating texture!");
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
                            tex
                        }
                        Err(e) => {
                            println!("Error presenting texture! {}", e.to_string());
                            panic!("rror presenting texture!");
                        }
                    };

                    window.view.canvas.present();
                }
            });
        });
    });
}
