use nalgebra::*;
use poncairepoint::{PoncairePoint, PoncaireWall};
use crate::utils::poncairepoint;
use serde::Deserialize;

use super::color::RGBColor;
#[derive(Clone, Debug, Deserialize)]
pub struct Hyperpoint(pub Point3<f64>);

type HyperTransMatrix = Matrix4<f64>;

impl From<PoncairePoint> for Hyperpoint {
    fn from(poncaire_point: PoncairePoint) -> Self {
        //Minkowski metric
        let norm_squared = PoncairePoint::minkowski_dot(&poncaire_point, &poncaire_point);
        Hyperpoint::new(
            (poncaire_point.0[0]*2.0)/(1.0-norm_squared),
            (poncaire_point.0[1]*2.0)/(1.0-norm_squared),
            (1.0+norm_squared)/(1.0-norm_squared)
        )
    }
}

impl Hyperpoint{
    /// Return the Minkowski inner product of the two vectors provided, where the
    /// last co-ordinate is interpreted as being time-like.
    pub fn minkowski_dot(a: &Hyperpoint, b: &Hyperpoint) -> f64{
        a.0[0] * b.0[0] +
        a.0[1] * b.0[1] -
        a.0[2] * b.0[2] 
    }

    pub fn new(x: f64, y: f64, z: f64) -> Hyperpoint{
        Hyperpoint{
            0: Point3::<f64>::new(x, y, z)
        }
    }

    pub fn transform(&mut self, matrix: HyperTransMatrix){
        self.0 = matrix.transform_point(&self.0);
    }

    pub fn rotate(&mut self, angle: f64){
        let rot = Rotation3::from_axis_angle(&Unit::new_normalize(Vector3::<f64>::new(0.0,0.0,1.0)), angle);
        self.0 = rot.transform_point(&self.0);
    }

    pub fn translate(&mut self, x: f64, y: f64){
        self.0[0] += x;
        self.0[1] += y;
        self.0[2] = (1.0+(self.0[0]).powi(2)+(self.0[1]).powi(2)).sqrt(); //ensure it stays on the hyperboloid
    }
    
}


#[derive(Clone, Debug, Deserialize)]
pub struct HyperWall {
    pub beginning: Hyperpoint,
    pub end: Hyperpoint,
    pub color: RGBColor
}

impl From<PoncaireWall> for HyperWall {
    fn from(poncaire_wall: PoncaireWall) -> HyperWall {
        HyperWall {
            beginning: poncaire_wall.beginning.into(), 
            end: poncaire_wall.end.into(), 
            color: poncaire_wall.color
        }
    }
}