extern crate sdl2;

mod emscripten;
mod game;
mod hyperbolic_renderer;
mod utils;
mod window;
use game::{hypermap::HyperMap, player::Player};
use game::Game;
use hyperbolic_renderer::Renderer;
use std::process::exit;
use std::time::Duration;
use utils::geometry::Point;
use utils::hyperpoint::Hyperpoint;
use window::event::Event;
use window::event::Keycode;
use window::Window;

fn main() {
    // Initialize the graphics and event handling.
    let mut window = Window::new();

    let map = HyperMap::new(include_str!("../assets/poncairemap2.json"));
    // Load the game and place the player within the map.
    let game = Game::new(map);

    // Initialize the renderer
    let mut renderer = Renderer::new(game, 1.0, 0.75, 100.0, 0.25);

    // Require a screen refresh after startup.
    let mut initial_run = true;

    // Run main loop on normal UI targets.
    #[cfg(not(target_os = "emscripten"))]
    loop {
        main_loop(&mut window, &mut renderer, &initial_run);
        initial_run = false;
    }

    // Run main loop on web targets.
    #[cfg(target_os = "emscripten")]
    {
        use emscripten::emscripten;
        emscripten::set_main_loop_callback(|| {
            main_loop(&mut window, &mut renderer, &initial_run);
            initial_run = false;
        });
    }
    //println!("{}", game.map.max_distance())
}

/// The main event handling loop.
fn main_loop(window: &mut Window, renderer: &mut Renderer, initial_run: &bool) {
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
        let movement_speed = 0.02;
        let rotation_speed = 0.02;

        match keycode {
            // Arrow up/down: Move player forward/backwards
            Keycode::Up => renderer.game.move_player(movement_speed),
            Keycode::Down => renderer.game.move_player(-movement_speed),

            // AWSD: Move player forward/backwards and strafe left/right
            Keycode::W => renderer.game.move_player(movement_speed),
            Keycode::S => renderer.game.move_player(-movement_speed),
            Keycode::D => renderer.game.strafe_player(movement_speed),
            Keycode::A => renderer.game.strafe_player(-movement_speed),

            // Arrow right/left: Rotate player
            Keycode::Right => {
                renderer.game.rotate_player(rotation_speed);
            }
            Keycode::Left => {
                renderer.game.rotate_player(-rotation_speed);
            }
        }
    }

    // Refresh screen if needed
    if needs_refresh {
        window
            .view
            .draw_canvas({ |mut canvas| renderer.render(&mut canvas) });
    }

    std::thread::sleep(Duration::from_millis(1000 / 60));
}
