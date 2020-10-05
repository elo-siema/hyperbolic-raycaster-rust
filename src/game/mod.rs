pub mod hypermap;
pub mod player;
pub mod map;

use crate::game::hypermap::*;
use crate::game::player::Player;

/// Represents the state of our game's virtual world
pub struct Game {
    /// The map of our virtual world
    pub map: HyperMap,
}

impl Game {
    /// Initializes a new game based on a given map and player.
    pub fn new(map: HyperMap) -> Game {
        Game { map }
    }

    /// Rotates the player's viewing angle with the given angle.
    ///
    /// # Parameters:
    ///		- `angle`:		The angle the player should rotated with (0…2π).
    pub fn rotate_player(&mut self, step: f64) {
        self.map.rotate(step);
    }

    /// Moves the player by the given distance in its current viewing direction. The player is not moved if it would collide with a wall.
    ///
    /// # Parameters:
    ///		- `distance:		The distance the player should be moved by.
    pub fn move_player(&mut self, distance: f64) {
        self.map.translate(distance, 0.0);
    }

    /// Moves the player by the given distance in its current viewing direction. The player is not moved if it would collide with a wall.
    ///
    /// # Parameters:
    ///		- `distance:		The distance the player should be moved by.
    pub fn strafe_player(&mut self, distance: f64) {
        self.map.translate(0.0, distance);
    }
}
