use crate::utils::poncairepoint;
use nalgebra::*;
use poncairepoint::{PoncairePoint, PoncaireWall};
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
            (poncaire_point.0[0] * 2.0) / (1.0 - norm_squared),
            (poncaire_point.0[1] * 2.0) / (1.0 - norm_squared),
            (1.0 + norm_squared) / (1.0 - norm_squared),
        )
    }
}

impl Hyperpoint {
    /// Return the Minkowski inner product of the two vectors provided, where the
    /// last co-ordinate is interpreted as being time-like.
    pub fn minkowski_dot(a: &Hyperpoint, b: &Hyperpoint) -> f64 {
        a.0[0] * b.0[0] + a.0[1] * b.0[1] - a.0[2] * b.0[2]
    }

    pub fn new(x: f64, y: f64, z: f64) -> Hyperpoint {
        Hyperpoint {
            0: Point3::<f64>::new(x, y, z),
        }
    }

    pub fn transform(&mut self, matrix: HyperTransMatrix) {
        self.0 = matrix.transform_point(&self.0);
    }

    pub fn rotate(&mut self, angle: f64) {
        
        let rot = Rotation3::from_axis_angle(
            &Unit::new_normalize(Vector3::<f64>::new(0.0, 0.0, 1.0)),
            angle,
        );
        self.0 = rot.transform_point(&self.0);
    }
    

    pub fn translate(&mut self, x: f64, y: f64) {
        
        //https://math.stackexchange.com/questions/1862340/what-are-the-hyperbolic-rotation-matrices-in-3-and-4-dimensions?newreg=0a895728ef9c48ad814e2f06eafb3862
        let coshb = f64::cosh(x);
        let sinhb = f64::sinh(x);
        let coshy = f64::cosh(-y);
        let sinhy = f64::sinh(-y);
        let translation1 = Matrix3::new(
            coshb, 0., sinhb,
            0., 1., 0.,
            sinhb, 0., coshb
        );
        let translation2 = Matrix3::new(
            1., 0., 0.,
            0., coshy, sinhy,
            0., sinhy, coshy
        ); // true form

        let translation = translation1 * translation2;
        self.0 = translation * &self.0;
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct HyperWall {
    pub beginning: Hyperpoint,
    pub end: Hyperpoint,
    pub color: RGBColor,
}

impl From<PoncaireWall> for HyperWall {
    fn from(poncaire_wall: PoncaireWall) -> HyperWall {
        HyperWall {
            beginning: poncaire_wall.beginning.into(),
            end: poncaire_wall.end.into(),
            color: poncaire_wall.color,
        }
    }
}
