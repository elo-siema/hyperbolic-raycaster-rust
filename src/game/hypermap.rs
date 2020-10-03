
use crate::utils::{color::RGBColor, hyperpoint::HyperWall, poncairepoint::PoncaireWall};
use crate::utils::geometry::Angle;
use crate::utils::geometry::Axis;
use crate::utils::geometry::Direction;
use crate::utils::geometry::Point;
use std::f64::*;
use serde::Deserialize; 

/// Represents the map of the 3D maze.
pub struct Map {
	/// The tiles of the map
	pub walls: Vec<HyperWall>,

}

/*pub struct Neighbors {
	
}*/

pub enum Direction5 {
	BACK,
	BACK_LEFT,
	FRONT_LEFT,
	FRONT_RIGHT,
	BACK_RIGHT
}


impl Map {
	/// Creates a new map from the given string.
	///
	/// # Parameters
    ///    - `map_string`:	A string representation of the map, whereas each line represents one row of the map and each character of a line represents a tile of a row.
	///						Use the characters R,G,B,Y,O to designate a wall with a certain color. Use spaces to designate empty tiles. Do not use tabs.
	pub fn new(map_string: &str) -> Map {
		// Go through the map line by line and create either tiles with a certain color or empty tiles.
		let walls: Vec<PoncaireWall> = serde_json::from_str(map_string).unwrap();
		let transformedWalls: Vec<HyperWall> = walls.into_iter().map(|w| w.into()).collect();
		println!("{:?}", transformedWalls);
		Map {walls: transformedWalls}
	}


	/// Returns the light intensity of a wall at a certain point depending on the viewing angle.
	///
	/// # Parameters:
	///		- point:		The point of the wall from which the light intensity is queried.
	///		- direction:	The direction the wall is viewn from
	pub fn light_intensity_for_wall(point: Point, direction: Angle) -> f64 {
		// Determine on which side of the wall the point resides.
		let closest_axis = point.closest_grid_line_axis();
		let viewing_direction = Direction::from_angle(&direction, &closest_axis);
		
		match closest_axis {
			// The ray hit a wall that is parallel to the x-axis
			Axis::X =>
				match viewing_direction {
					Direction::Increasing => 1.0,
					Direction::Decreasing => 0.6
				},

			// The ray hit a wall that is parallel to the y-axis
			Axis::Y =>
				match viewing_direction {
					Direction::Increasing => 0.4,
					Direction::Decreasing => 0.8
				}
		}
	}	
}