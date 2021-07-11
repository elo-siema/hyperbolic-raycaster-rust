use crate::utils::{hyperpoint::HyperWall, poincarepoint::PoincareWall};

/// Represents the map in the Minkowski hyperboloid model.
pub struct HyperMap {
    /// Walls of the map.
    walls: Vec<HyperWall>,
}

impl HyperMap {
    /// Creates a new map from the given JSON string.
    ///
    /// # Parameters
    ///    - `map_string`:	A JSON representation of the map, an array of PoincareWalls.
    pub fn new(map_string: &str) -> HyperMap {
        // Parse JSON to PoincareWalls.
        let walls: Vec<PoincareWall> = serde_json::from_str(map_string).unwrap();

        // Scrapped idea - representing the walls as a set sorted by distance to origin.
        // Would need to be checked and resorted every frame.
        /*let mut transformedWalls: BTreeSet<HyperWall> = BTreeSet::<HyperWall>::new();
        for wall in walls {
            let transformed : HyperWall= wall.into();
            transformedWalls.insert(transformed);
        }*/

        // Then transform them into HyperWalls as internal representation.
        // This is done so it's easier to do transformations on the walls.
        let mut transformed_walls: Vec<HyperWall> = walls.into_iter().map(|w| w.into()).collect();

        // Sort by distance to origin.
        transformed_walls.sort_unstable();
        HyperMap {
            walls: transformed_walls,
        }
    }

    /// Returns iterator of HyperWall references.
    pub fn get_walls_iter(&self) -> impl Iterator<Item = &HyperWall> {
        self.walls.iter()
    }

    /// Returns iterator of PoincareWall references.
    pub fn get_walls_as_poincare(&self) -> Vec<PoincareWall> {
        let wallsp: Vec<PoincareWall> = self.walls.iter().map(|hw| hw.clone().into()).collect(); //todo: don't clone
                                                                                                 //not sorting, because we're iterating through them all anyway
                                                                                                 //wallsp.sort_by(|a, b| a.distance_to_origin().partial_cmp(&b.distance_to_origin()).unwrap() );
        wallsp
    }

    /// Rotate all walls around an origin.
    pub fn rotate(&mut self, step: f64) {
        for wall in &mut self.walls {
            wall.beginning.rotate(step);
            wall.end.rotate(step);
        }
        // Keep walls sorted
        self.walls.sort_unstable();
    }

    /// Move all walls along the x and y axes.
    pub fn translate(&mut self, x: f64, y: f64) {
        for wall in &mut self.walls {
            wall.beginning.translate(x, y);
            wall.end.translate(x, y);
        }
        // Keep walls sorted
        self.walls.sort_unstable();
    }
}
