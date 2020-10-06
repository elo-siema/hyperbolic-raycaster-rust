use crate::utils::geometry::Angle;
use crate::utils::geometry::Axis;
use crate::utils::geometry::Direction;
use crate::utils::geometry::Point;
use crate::utils::{color::RGBColor, hyperpoint::HyperWall, poncairepoint::PoncaireWall, point::*};
use std::{collections::BTreeSet, f64::*};

//use super::map::Map;

/// Represents the map of the 3D maze.
pub struct HyperMap {
    /// The tiles of the map, sorted by distance to origin.
    walls: Vec<HyperWall>,
}

pub struct WallIterator {

}


impl HyperMap {
    /// Creates a new map from the given string.
    ///
    /// # Parameters
    ///    - `map_string`:	A string representation of the map, whereas each line represents one row of the map and each character of a line represents a tile of a row.
    ///						Use the characters R,G,B,Y,O to designate a wall with a certain color. Use spaces to designate empty tiles. Do not use tabs.
    pub fn new(map_string: &str) -> HyperMap {
        // Go through the map line by line and create either tiles with a certain color or empty tiles.
        let walls: Vec<PoncaireWall> = serde_json::from_str(map_string).unwrap();
        /*let mut transformedWalls: BTreeSet<HyperWall> = BTreeSet::<HyperWall>::new();
        for wall in walls {
            let transformed : HyperWall= wall.into();
            transformedWalls.insert(transformed);
        }*/
        let mut transformedWalls: Vec<HyperWall> = walls.into_iter().map(|w| w.into()).collect();
        transformedWalls.sort_unstable();
        println!("{:?}", transformedWalls);
        HyperMap {
            walls: transformedWalls,
        }
    }

    //Guaranteed sorted by distance from origin descending.
    pub fn get_walls_iter(&self) -> impl Iterator<Item = &HyperWall> {
        self.walls.iter()
    }

    //Guaranteed sorted by distance from origin descending.
    pub fn get_walls_as_poncaire(&self) -> Vec<PoncaireWall> {
        self.walls.iter().map(|hw| hw.clone().into()).collect() //todo: don't clone
    }

    pub fn rotate(&mut self, step: f64) {
        for wall in &mut self.walls {
            wall.beginning.rotate(step);
            wall.end.rotate(step);
        }
        self.walls.sort_unstable();
    }

    pub fn translate(&mut self, x: f64, y: f64) {
        for wall in &mut self.walls {
            wall.beginning.translate(x, y);
            wall.end.translate(x, y);
        }
        self.walls.sort_unstable();
    }
}
