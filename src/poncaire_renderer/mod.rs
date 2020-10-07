use std::{cell::RefCell, rc::Rc};

use crate::utils::color::RGBColor;
use crate::window::canvas::Canvas;
use crate::{game::Game, utils::poncairepoint::PoncairePoint, utils::poncairepoint::PoncaireWall};
use line_drawing::Bresenham;

/// Draws a top-down view on a Poncaire disk.
pub struct Renderer {
    /// The state of the virtual world to be rendered
    pub game: Game,

    /// The size of the physical computer display in relation to a grid field
    pub relative_screen_size: f64,

    /// The focal length used for determining the window angle
    pub focal_length: f64,

    /// The radius around the player where objects should appear illuminated
    pub illumination_radius: f64,

    /// The minimum environment light of the scene
    pub minimum_light: f64,
}

impl Renderer {
    /// Initializes the renderer with a map, a player and a focal length that should be used for rendering.
    ///
    /// # Parameters:
    /// 	- game:						The virtual world state (i.e. the game's map and player position)
    ///		- relative_screen_size:		The size of the physical computer display in relation to a grid field
    ///  	- focal_length:				A focal length that should be used for rendering.
    ///	 	- illumination_radius:		The radius around the player where objects should appear illuminated.
    ///	 	- minimum_öight:			The minimum environment light of the scene.
    ///
    pub fn new(
        game: Game,
        relative_screen_size: f64,
        focal_length: f64,
        illumination_radius: f64,
        minimum_light: f64,
    ) -> Renderer {
        Renderer {
            game,
            relative_screen_size,
            focal_length,
            illumination_radius,
            minimum_light,
        }
    }

    /// Renders one frame into a canvas.
    ///
    /// # Parameters:
    ///		- canvas		The canvas that should be drawn to.
    pub fn render(&self, canvas: &mut Canvas) {
        //let helper: PoncairePoint = self.game.borrow().map.walls[0].beginning.clone().into();
        //println!("{:?}", helper);
        //for a in self.game.map

        self.game
            .map
            .get_walls_as_poncaire()
            .iter()
            .map(|w| w.clone().into())
            .for_each(|wall: PoncaireWall| {
                self.draw_wall(&wall, canvas);
            });
    }

    fn draw_wall(&self, wall: &PoncaireWall, canvas: &mut Canvas) {
        let start =
            self.translate_to_canvas_coords(wall.beginning.0[0], wall.beginning.0[1], canvas);
        let end = self.translate_to_canvas_coords(wall.end.0[0], wall.end.0[1], canvas);

        for (x, y) in Bresenham::new(start, end) {
            canvas.draw_pixel(x as usize, y as usize, &wall.color);
        }
    }

    ///expects x and y between -1:1
    fn draw_point_of_a_disc(&self, x: f64, y: f64, color: &RGBColor, canvas: &mut Canvas) {
        let (outputX, outputY) = self.translate_to_canvas_coords(x, y, canvas);
        canvas.draw_pixel_big(outputX as usize, outputY as usize, &color);
    }

    fn translate_to_canvas_coords(&self, x: f64, y: f64, canvas: &Canvas) -> (i32, i32) {
        let window_height = canvas.height();
        let window_width = canvas.width();
        let left_pad = (window_width - window_height) / 2;

        (
            ((x + 1.0) * 250.0) as i32 + 150,
            ((y + 1.0) * 250.0) as i32 + 30,
        )
    }
}
