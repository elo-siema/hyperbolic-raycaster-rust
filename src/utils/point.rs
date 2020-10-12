use std::cmp::*;

pub trait Point {
    fn distance_to_origin(&self) -> f64;
    fn distance_to(&self, to: &Self) -> f64;
    fn minkowski_dot(a: &Self, b: &Self) -> f64;
    fn new_at_origin() -> Self;
}

pub trait Wall: Ord + Eq + PartialEq + PartialOrd {
    fn distance_to_closest_point(&self) -> f64;
    fn intersection(&self, angle: f64) -> Option<f64>;
}
