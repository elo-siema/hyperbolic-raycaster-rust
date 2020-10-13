use crate::utils::color::RGBColor;
use crate::window::canvas::Canvas;
use crate::{game::Game, utils::poincarepoint::PoincareWall};

enum Hit {
    /// The ray hit a wall with a given color at a given distance.
    Wall { color: RGBColor, distance: f64 },
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
    pub field_of_vision: f64,
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
            field_of_vision: std::f64::consts::PI / 2.0,
        }
    }

    /// Renders one frame into a canvas.
    ///
    /// # Parameters:
    ///		- canvas		The canvas that should be drawn to.
    pub fn render(&self, canvas: &mut Canvas) {
        let walls: Vec<PoincareWall> = self.game.map.get_walls_as_poincare();
        for column in 0..canvas.width() {
            self.render_column(column, canvas, &walls);
        }
    }

    fn render_column(&self, column: usize, canvas: &mut Canvas, walls: &[PoincareWall]) {
        // Cast the ray to find a nearby wall
        let scanning_result = self.cast_ray(column, canvas.width(), walls);

        // Draw scanning result to the canvas
        self.draw_hit(scanning_result, column, canvas);
    }

    fn cast_ray(&self, column: usize, max_column: usize, walls: &[PoincareWall]) -> Option<Hit> {
        // Determine the absolute angle of the ray
        let angle = self.ray_angle(column, max_column);
        let mut closest_hit: Option<Hit> = None;

        walls.iter().for_each(|wall| {
            match wall.find_distance_of_intersection_with_ray(angle) {
				// Ray hit wall
                Some(distance) => {
                    // Fix the calculated distance to correct the fisheye effect
                    let projected_distance = distance * angle.cos();

                    // Apply some lighting to the wall's color
                    let distance_light_intensity = (1.0
                        - projected_distance / self.illumination_radius)
                        .max(self.minimum_light)
                        .min(1.0);
                    let illuminated_color = wall.color.adjust_light_intensity(
                        distance_light_intensity
                    );

                    // Pass the result
                    match &closest_hit {
						// Ignore case when found hit is farther than closest hit up to this point
						Some(Hit::Wall { color: _, distance }) if projected_distance >= *distance => (),
						
						// Update closest hit if it's None, or if we found a closer hit
                        _ => {
                            closest_hit = Some(Hit::Wall {
                                color: illuminated_color,
                                distance: projected_distance,
                            })
                        }
                    }
				}
				// Ray did not hit wall
                None => (),
            }
        });
        closest_hit
    }

    fn draw_hit(&self, hit: Option<Hit>, column: usize, canvas: &mut Canvas) {
        match hit {
            // We did not found a wall, just draw an empty space
            None => self.draw_wall(0.0, RGBColor::black(), canvas, column),

            Some(Hit::Wall { color, distance }) => {
				// Determine the visual height of the wall on the screen (normalized to the screen's height)
				// 0.1 found by experiment. Works well with the scale of things on the Poincare disk coordinates
                let normalized_wall_height = 0.1 / distance; //todo:: Allow different wall heights

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
}