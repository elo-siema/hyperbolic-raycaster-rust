mod ray;

use crate::{hyperbolic_renderer::ray::*, utils::color};
use crate::utils::color::RGBColor;
use crate::window::canvas::Canvas;
use crate::{game::Game, utils::poncairepoint::PoncairePoint, utils::poncairepoint::PoncaireWall};
use line_drawing::Bresenham;
use nalgebra::Point2;

enum Hit {
	/// The ray never hit a wall
	None,

	/// The ray hit a wall with a given color at a given distance.
	Wall {color: RGBColor, distance: f64}
}
/// Raycaster in hyperbolic space.
pub struct Renderer {
    /// The state of the virtual world to be rendered
    pub game: Game,

    /// The radius around the player where objects should appear illuminated
	pub illumination_radius: f64,
	pub relative_screen_size: f64,
	pub focal_length: f64,
	

    /// The minimum environment light of the scene
	pub minimum_light: f64,
	
	pub player_height: f64,
	pub field_of_vision: f64
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
			player_height: 0.05,
			field_of_vision: std::f64::consts::PI/2.0
        }
    }

    /// Renders one frame into a canvas.
    ///
    /// # Parameters:
    ///		- canvas		The canvas that should be drawn to.
    pub fn render(&self, canvas: &mut Canvas) {
		for column in 0..canvas.width() {
			self.render_column(column, canvas);
		}
	}
	
	fn render_column(&self, column: usize, canvas: &mut Canvas) {
		// Cast the ray to find a nearby wall
		let scanning_result = self.cast_ray(column, canvas.width());

		// Draw scanning result to the canvas
		self.draw_hit(scanning_result, column, canvas);
	}

	fn cast_ray(&self, column: usize, max_column: usize) -> Hit {
		// Determine the absolute angle of the ray
		let relative_angle = self.ray_angle(column, max_column);
		let absolute_angle = relative_angle;

		// Create the ray
		let mut ray = Ray::new(Point2::<f64>::new(0.0,0.0), absolute_angle);

		// Grow the ray stepy by step. Grow it until we either hit a wall or reached the maximal possible distance inside our map
		while ray.length <= 2 as f64 {
			ray = ray.grow();

			
					// Fix the calculated distance to correct the fisheye effect
					let projected_distance = ray.length * relative_angle.cos();	
						
					// Apply some lighting to the wall's color
					let wall_light_intensity = Map::light_intensity_for_wall(ray.end, ray.angle);
					let distance_light_intensity = (1.0 - ray.length/self.illumination_radius).max(self.minimum_light).min(1.0);
					let illuminated_color = color.adjust_light_intensity(distance_light_intensity * wall_light_intensity);

					// Pass the result
					return Hit::Wall {color: illuminated_color, distance: projected_distance}
				
				
			
		}	

		// The ray casting reached the outer bounds of our map. We never hit a wall...
		return Hit::None;
	}

		fn draw_hit(&self, hit: Hit, column: usize, canvas: &mut Canvas) {
		match hit {
			// We did not found a wall, just draw an empty space
			Hit::None => self.draw_wall(0.0, RGBColor::black(), canvas, column),
			
			Hit::Wall {color, distance} => {
				// Determine the visual height of the wall on the screen (normalized to the screen's height)
				let normalized_wall_height = 1.0 / distance;

				// Finally: Draw the wall for the current view position…
				self.draw_wall(normalized_wall_height, color, canvas, column)
			}
		}
	}

	/// Draws a column of a wall for the given view position.
	///
	///	# Parameters:
	///  	- wall_height:	The visible height of a wall segment to be drawn (0: no wall, >=1: full view height).
	///  	- color:		The color of the wall to be drawn.
	///  	- canvas:		The canvas that should be used for drawing.
	///  	- column:		The current view column to be drawn.
	fn draw_wall(&self, wall_height: f64, color: RGBColor, canvas: &mut Canvas, column: usize) {
		let window_height = canvas.height();
		let limited_wall_height = wall_height.min(1.0);
		let view_wall_height = ((window_height as f64) * limited_wall_height) as usize;

		let wall_top = (window_height - view_wall_height) / 2;
		let wall_bottom = wall_top + view_wall_height;

		// Draw the black ceiling
		for y in 0..wall_top {
			canvas.draw_pixel(column, y, &RGBColor::black());
		}

		// Draw the wall (if anything is visible)
		for y in wall_top..wall_bottom {
			canvas.draw_pixel(column as usize, y as usize, &color);
		}

		// Draw the floor as grey gradient
		for y in wall_bottom..window_height {
			let gradient_position = y as f64 / window_height as f64;
			let gradient_color = RGBColor::dark_gray().adjust_light_intensity(gradient_position);
			canvas.draw_pixel(column as usize, y as usize, &gradient_color);
		}
	}

	/// Determines the angle of a scanning ray for drawing the given column on a view with the given width.
	/// The ray should be casted from the given player's using its position, viewing direction and the current focal length.
	///
	///	# Parameters:
	///		- column:	The current view column to be drawn (which must be less than the view's width).
	///		- width:	The width of the view.
	fn ray_angle(&self, column: usize, max_column: usize) -> f64 {
		let relative_position = ((column as f64) / (max_column as f64)) - 0.5;
		let virtual_screen_position = relative_position * self.relative_screen_size;
		return (virtual_screen_position / self.focal_length).atan();
	}

	/*
    fn draw_wall(&self, wall: &PoncaireWall, canvas: &mut Canvas) {
        let start =
            self.translate_to_canvas_coords(wall.beginning.0[0], wall.beginning.0[1], canvas);
        let end = self.translate_to_canvas_coords(wall.end.0[0], wall.end.0[1], canvas);

        for (x, y) in Bresenham::new(start, end) {
            canvas.draw_pixel(x as usize, y as usize, &wall.color);
        }
    }
*/
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

/*mod ray;

use crate::game::Game;
use crate::game::map::Tile;
use crate::game::map::TilePosition;
use crate::game::map::Map;
use crate::hyperbolic_renderer::ray::Ray;
use crate::utils::color::RGBColor;
use crate::utils::geometry::Angle;
use crate::window::canvas::Canvas;

/// Draws a 3D scene for a given map and a player within the map.
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
	pub minimum_light: f64
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
	pub fn new(game: Game, relative_screen_size: f64, focal_length: f64, illumination_radius: f64, minimum_light: f64) -> Renderer {
		Renderer {game, relative_screen_size, focal_length, illumination_radius, minimum_light}
	}

	/// Renders one frame into a canvas.
	///
	/// # Parameters:
	///		- canvas		The canvas that should be drawn to.
	pub fn render(&self, canvas: &mut Canvas) {
		for column in 0..canvas.width() {
			self.render_column(column, canvas);
		}
	}

	/// Renders one pixel column of a frame into a canvas.
	///
	/// # Parameters:
	///		- column:		The pixel column of the canvas that should be rendered
	///		- canvas:		The canvas that should be drawn to.
	fn render_column(&self, column: usize, canvas: &mut Canvas) {
		// Cast the ray to find a nearby wall
		let scanning_result = self.cast_ray(column, canvas.width());

		// Draw scanning result to the canvas
		self.draw_hit(scanning_result, column, canvas);
	}
}

/// Describes the result of a casted ray
enum Hit {
	/// The ray never hit a wall
	None,

	/// The ray hit a wall with a given color at a given distance.
	Wall {color: RGBColor, distance: f64}
}

// Methods related to ray casting
impl Renderer {
	/// Casts a ray from the player's position for a given column on the view and returns what the ray scanned at its end.
	///
	/// # Parameters:
	///		- column:			The column that should be drawn
	///		- width:			The largest column number that could be drawn
	fn cast_ray(&self, column: usize, max_column: usize) -> Hit {
		// Determine the absolute angle of the ray
		let relative_angle = self.ray_angle(column, max_column);
		let absolute_angle = relative_angle + self.game.player.direction;

		// Create the ray
		let mut ray = Ray::new(self.game.player.position.clone(), absolute_angle);

		// Grow the ray stepy by step. Grow it until we either hit a wall or reached the maximal possible distance inside our map
		while ray.length <= self.game.map.max_distance() as f64 {
			ray = ray.grow();

			let tile = self.game.map.tile(&TilePosition::new(&ray.end, &ray.angle));
			match tile.unwrap().get_color() { // TODO
				crate::game::map::TileContent::Wall(color) => {
					// Fix the calculated distance to correct the fisheye effect
					let projected_distance = ray.length * relative_angle.cos();	
						
					// Apply some lighting to the wall's color
					let wall_light_intensity = Map::light_intensity_for_wall(ray.end, ray.angle);
					let distance_light_intensity = (1.0 - ray.length/self.illumination_radius).max(self.minimum_light).min(1.0);
					let illuminated_color = color.adjust_light_intensity(distance_light_intensity * wall_light_intensity);

					// Pass the result
					return Hit::Wall {color: illuminated_color, distance: projected_distance}
				},
				

			    crate::game::map::TileContent::Empty => {}
			}
		}	

		// The ray casting reached the outer bounds of our map. We never hit a wall...
		return Hit::None;
	}

	/// Determines the angle of a scanning ray for drawing the given column on a view with the given width.
	/// The ray should be casted from the given player's using its position, viewing direction and the current focal length.
	///
	///	# Parameters:
	///		- column:	The current view column to be drawn (which must be less than the view's width).
	///		- width:	The width of the view.
	fn ray_angle(&self, column: usize, max_column: usize) -> Angle {
		let relative_position = ((column as f64) / (max_column as f64)) - 0.5;
		let virtual_screen_position = relative_position * self.relative_screen_size;
		return (virtual_screen_position / self.focal_length).atan();
	}
}

// Methods related to drawing
impl Renderer {
	/// Draws the given view column for the result of a particular ray casting operation to a given canvas
	fn draw_hit(&self, hit: Hit, column: usize, canvas: &mut Canvas) {
		match hit {
			// We did not found a wall, just draw an empty space
			Hit::None => self.draw_wall(0.0, RGBColor::black(), canvas, column),
			
			Hit::Wall {color, distance} => {
				// Determine the visual height of the wall on the screen (normalized to the screen's height)
				let normalized_wall_height = 1.0 / distance;

				// Finally: Draw the wall for the current view position…
				self.draw_wall(normalized_wall_height, color, canvas, column)
			}
		}
	}

	/// Draws a column of a wall for the given view position.
	///
	///	# Parameters:
	///  	- wall_height:	The visible height of a wall segment to be drawn (0: no wall, >=1: full view height).
	///  	- color:		The color of the wall to be drawn.
	///  	- canvas:		The canvas that should be used for drawing.
	///  	- column:		The current view column to be drawn.
	fn draw_wall(&self, wall_height: f64, color: RGBColor, canvas: &mut Canvas, column: usize) {
		let window_height = canvas.height();
		let limited_wall_height = wall_height.min(1.0);
		let view_wall_height = ((window_height as f64) * limited_wall_height) as usize;

		let wall_top = (window_height - view_wall_height) / 2;
		let wall_bottom = wall_top + view_wall_height;

		// Draw the black ceiling
		for y in 0..wall_top {
			canvas.draw_pixel(column, y, &RGBColor::black());
		}

		// Draw the wall (if anything is visible)
		for y in wall_top..wall_bottom {
			canvas.draw_pixel(column as usize, y as usize, &color);
		}

		// Draw the floor as grey gradient
		for y in wall_bottom..window_height {
			let gradient_position = y as f64 / window_height as f64;
			let gradient_color = RGBColor::dark_gray().adjust_light_intensity(gradient_position);
			canvas.draw_pixel(column as usize, y as usize, &gradient_color);
		}
	}
}
*/