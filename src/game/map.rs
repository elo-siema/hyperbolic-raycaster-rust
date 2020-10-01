
use crate::utils::color::RGBColor;
use crate::utils::geometry::Angle;
use crate::utils::geometry::Axis;
use crate::utils::geometry::Direction;
use crate::utils::geometry::Point;
use serde::Deserialize; 

/// Represents the map of the 3D maze.
pub struct Map {
	/// The tiles of the map
	tiles: Vec<Tile>,

	/// The longest distance between two points within the map
	max_distance: usize
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


pub struct CircularIterator<'a, T> {
	vec: &'a Vec<T>,
	offset: usize,
	current_count: usize
}

impl<T> Iterator for CircularIterator<'_, T> 
	where T: Copy {
	type Item = T;

	
	///	Will return vec.size elements starting from offset
	fn next(&mut self) -> Option<Self::Item> {
		let result;
        if self.current_count >= self.vec.len() {
			result = None;
		} else {
			result = Some(self.vec[self.current_count % self.vec.len()]);
		}

		self.current_count += 1;

		result
    }
}
#[derive(Deserialize, Debug)]
pub struct Tile {
	neighbors: Vec<Option<usize>>,
	content: i32
}

impl Tile {
	pub fn iter(&self) -> CircularIterator<Option<usize>>{
		CircularIterator {
			vec: &self.neighbors,
			offset: 0,
			current_count: 0
		}
	}

	pub fn iter_from(&self, offset: usize) -> CircularIterator<Option<usize>>{
		CircularIterator {
			vec: &self.neighbors,
			offset: offset,
			current_count: 0
		}
	}

	pub fn get_color(&self) -> TileContent {
		match self.content {
			1 => TileContent::Wall(RGBColor::red()),
			2 => TileContent::Wall(RGBColor::green()),
			3 => TileContent::Wall(RGBColor::blue()),
			4 => TileContent::Wall(RGBColor::yellow()),
			_ => TileContent::Empty
		}
	}
}

#[derive(PartialEq, Eq, Clone)]
pub enum TileContent {
	Empty,
	Wall(RGBColor)
}

impl Map {
	/// Creates a new map from the given string.
	///
	/// # Parameters
    ///    - `map_string`:	A string representation of the map, whereas each line represents one row of the map and each character of a line represents a tile of a row.
	///						Use the characters R,G,B,Y,O to designate a wall with a certain color. Use spaces to designate empty tiles. Do not use tabs.
	pub fn new(map_string: &str) -> Map {
		// Go through the map line by line and create either tiles with a certain color or empty tiles.
		let tiles: Vec<Tile> = serde_json::from_str(map_string).unwrap();

		// Since our map is rectangular the longest possible distance can be never longer than the sum of the height or width (https://en.wikipedia.org/wiki/Triangle_inequality).
		//let height = tiles.len();
		//let width = tiles.iter().fold(0, {|max_count, line| line.len().max(max_count)});
		//let max_distance = height + width;
		//we've stopped thinking in those terms
		let max_distance = 5;

		Map {tiles, max_distance}
	}

	/// Returns the contents of an tile inside the map.
	///
	/// # Parameters
	/// 	- `point`:	The position of the tile. If the position is outside the map, an empty field is returned.
	pub fn tile(&self, position: &TilePosition) -> Option<Tile> {
		/*if position.y < 0 || position.y as usize >= self.tiles.len() {
			return Tile::Empty;
		}

		let line = &self.tiles[position.y as usize];
		if position.x < 0 || position.x as usize >= line.len() {
			return Tile::Empty;
		}

		return line[position.x as usize].clone();*/
		None
	}

	/// The longest distance between two points that can exist within the map
	pub fn max_distance(&self) -> usize {
		self.max_distance
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

/// The position of a tile within the map
pub struct TilePosition {
	x: isize,
	y: isize
}

impl TilePosition {
	/// Rounds a point to a tile position. Makes sure that ambiguous points - i.e. points between two tiles - are properly rounded using the given angle.
	///
	/// # Parameters:
	///		- point:		The point that should be converted to a tile position.
	///		- angle:		An angle used to resolve ambiguous points.
	pub fn new(point: &Point, angle: &Angle) -> TilePosition {
		let x = TilePosition::component_from_point(point, Axis::X, angle);
		let y = TilePosition::component_from_point(point, Axis::Y, angle);

		TilePosition {x, y}
	}

	/// Rounds a single coordinate of a point to a single coordinate of a tile position. Makes sure that ambiguous coordinates - i.e. points between two
	/// tiles - are properly rounded using the given angle.
	///
	/// # Parameters:
	///		- point:		The point from which a component should be converted to a tile position component.
	///		- axis:			The axis the point's component should be taken from
	///		- angle:		An angle used to resolve ambiguous points.
	fn component_from_point(point: &Point, axis: Axis, angle: &Angle) -> isize {
		let mut point_component = point.component(&axis);

		//if point_component == point_component as i64 as f64 {
		if point_component.fract() == 0.0 {
			match Direction::from_angle(angle, &axis) {
				Direction::Increasing => {}
				Direction::Decreasing => { point_component -= 1.0; }
			}
		}

		point_component as isize
	}
}
