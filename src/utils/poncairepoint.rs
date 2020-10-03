use hyperpoint::{HyperWall, Hyperpoint};
use nalgebra::*;
use serde::Deserialize;

use crate::utils::hyperpoint;

use super::color::RGBColor;
#[derive(Clone, Debug, Deserialize)]
pub struct PoncairePoint(pub Point2<f64>);

trait ToHyper {
    fn ToPoncaire(&self) -> Hyperpoint;
}

impl From<Hyperpoint> for PoncairePoint {
    fn from(hyperpoint: Hyperpoint) -> Self {
            let denom = hyperpoint.0[2] + 1.0;
            PoncairePoint::new(hyperpoint.0[0]/denom, hyperpoint.0[1]/denom)
    }
}

impl PoncairePoint{
    /// Return the Minkowski inner product of the two vectors provided, where the
    /// last co-ordinate is interpreted as being time-like.
    pub fn minkowski_dot(a: &PoncairePoint, b: &PoncairePoint) -> f64{
        a.0[0] * b.0[0] -
        a.0[1] * b.0[1] 
    }

    pub fn new(x: f64, y: f64) -> PoncairePoint{
        PoncairePoint{
            0: Point2::<f64>::new(x, y)
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct PoncaireWall {
    pub beginning: PoncairePoint,
    pub end: PoncairePoint,
    pub color: RGBColor
}
impl From<HyperWall> for PoncaireWall {
    fn from(hyperwall: HyperWall) -> PoncaireWall {
        PoncaireWall {
            beginning: hyperwall.beginning.into(), 
            end: hyperwall.end.into(), 
            color: hyperwall.color
        }
    }
}