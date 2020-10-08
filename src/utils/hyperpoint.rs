use std::cmp;

use crate::utils::*;
use cmp::Ordering;
use nalgebra::*;
use point::{Point, Wall};
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
        Hyperpoint::new_with_z(
            (poncaire_point.0[0] * 2.0) / (1.0 - norm_squared),
            (poncaire_point.0[1] * 2.0) / (1.0 - norm_squared),
            (1.0 + norm_squared) / (1.0 - norm_squared),
        )
    }
}

impl Hyperpoint {
    pub fn new_with_z(x: f64, y: f64, z: f64) -> Hyperpoint {
        Hyperpoint {
            0: Point3::<f64>::new(x, y, z),
        }
    }
    pub fn new(x: f64, y: f64) -> Hyperpoint {
        let z = (1.0+x.powi(2)+y.powi(2)).sqrt();
        Hyperpoint {
            0: Point3::<f64>::new(x, y, z),
        }
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
        ); 

        let translation = translation1 * translation2;
        self.0 = translation * &self.0;
    }
}

impl point::Point for Hyperpoint{
    /// Return the Minkowski inner product of the two vectors provided, where the
    /// last co-ordinate is interpreted as being time-like.
    fn minkowski_dot(a: &Hyperpoint, b: &Hyperpoint) -> f64 {
        a.0[0] * b.0[0] + a.0[1] * b.0[1] - a.0[2] * b.0[2]
    }

    fn distance_to_origin(&self) -> f64 {
        let minkowski_bilinear: f64 = 
            self.0[2];
        minkowski_bilinear.acosh()
    }

    fn new_at_origin() -> Self { 
        Hyperpoint::new_with_z(0., 0., 1.)
    }

    fn distance_to(&self, to: &Self) -> f64 {
        let minkowski_bilinear: f64 = 
            self.0[2] * to.0[2] -
            self.0[1] * to.0[1] -
            self.0[0] * to.0[0];
        minkowski_bilinear.acosh()
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct HyperWall {
    pub beginning: Hyperpoint,
    pub end: Hyperpoint,
    pub color: RGBColor,
}

impl HyperWall {
    fn find_plane_through_2_points_and_origin(p1: Hyperpoint, p2: Hyperpoint) -> (f64, f64, f64) {
        let (ax, ay, az): (f64, f64, f64) = (p1.0[0], p1.0[1], p1.0[2]); 
        let (bx, by, bz): (f64, f64, f64) = (p1.0[0], p1.0[1], p1.0[2]); 
        let (cx, cy, cz) = (0., 0., 0.);

        let a = (by-ay)*(cz-az)-(cy-ay)*(bz-az);
        let b = (bz-az)*(cx-ax)-(cz-az)*(bx-ax);
        let c = (bx-ax)*(cy-ay)-(cx-ax)*(by-ay);

        (a,b,c)
    }
}

impl Wall for HyperWall {
    fn distance_to_closest_point(&self) -> f64 {
        let distA = self.beginning.distance_to_origin();
        let distB = self.end.distance_to_origin();

        distA.min(distB)
    }

    fn intersection(&self, angle: f64) -> Option<f64> {
        todo!()
    }
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

impl Ord for HyperWall {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Eq for HyperWall {
    
}

impl PartialEq for HyperWall {
    fn eq(&self, other: &Self) -> bool {
        self.distance_to_closest_point()
            .eq(&other.distance_to_closest_point())
    }
}

impl PartialOrd for HyperWall {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.distance_to_closest_point()
            .partial_cmp(&other.distance_to_closest_point())
    }
}