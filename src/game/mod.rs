pub mod hypermap;
pub mod player;

use crate::game::hypermap::*;
use crate::game::player::Player;

/// Represents the state of our game's virtual world
pub struct Game {
    /// The map of our virtual world
    pub map: Map,
}

impl Game {
    /// Initializes a new game based on a given map and player.
    pub fn new(map: Map) -> Game {
        Game { map }
    }

    /// Rotates the player's viewing angle with the given angle.
    ///
    /// # Parameters:
    ///		- `angle`:		The angle the player should rotated with (0…2π).
    pub fn rotate_player(&mut self, step: f64) {
        for wall in &mut self.map.walls {
            wall.beginning.rotate(step);
            wall.end.rotate(step);
        }
    }

    /// Moves the player by the given distance in its current viewing direction. The player is not moved if it would collide with a wall.
    ///
    /// # Parameters:
    ///		- `distance:		The distance the player should be moved by.
    pub fn move_player(&mut self, distance: f64) {
        for wall in &mut self.map.walls {
            wall.beginning.translate(0.0, distance);
            wall.end.translate(0.0, distance);
        }
    }

    /// Moves the player by the given distance in its current viewing direction. The player is not moved if it would collide with a wall.
    ///
    /// # Parameters:
    ///		- `distance:		The distance the player should be moved by.
    pub fn strafe_player(&mut self, distance: f64) {
        for wall in &mut self.map.walls {
            wall.beginning.translate(distance, 0.0);
            wall.end.translate(distance, 0.0);
        }
    }
}
