use hyperpoint::{HyperWall, Hyperpoint};
use nalgebra::*;
use serde::Deserialize;

use crate::utils::hyperpoint;

use super::{color::RGBColor, point};
#[derive(Clone, Debug, Deserialize)]
pub struct PoncairePoint(pub Point2<f64>);

impl From<Hyperpoint> for PoncairePoint {
    fn from(hyperpoint: Hyperpoint) -> Self {
        let denom = hyperpoint.0[2] + 1.0;
        PoncairePoint::new(hyperpoint.0[0] / denom, hyperpoint.0[1] / denom)
    }
}

impl PoncairePoint {
    pub fn new(x: f64, y: f64) -> PoncairePoint {
        PoncairePoint {
            0: Point2::<f64>::new(x, y),
        }
    }
}

impl point::Point for PoncairePoint {
    
    /// Return the Minkowski inner product of the two vectors provided, where the
    /// last co-ordinate is interpreted as being time-like.
    fn minkowski_dot(a: &PoncairePoint, b: &PoncairePoint) -> f64 {
        a.0[0] * b.0[0] - a.0[1] * b.0[1]
    }

    fn distance_to_origin(&self) -> f64 {
        let euclidian_distance = 
            (self.0[0].powi(2) + self.0[1].powi(2))
            .sqrt();
        f64::ln(euclidian_distance)
    }

    fn new_at_origin() -> Self {
        todo!()
    }

    fn distance_to(&self, to: &Self) -> f64 {
        todo!() //fuck that
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct PoncaireWall {
    pub beginning: PoncairePoint,
    pub end: PoncairePoint,
    pub color: RGBColor,
}
impl From<HyperWall> for PoncaireWall {
    fn from(hyperwall: HyperWall) -> PoncaireWall {
        PoncaireWall {
            beginning: hyperwall.beginning.into(),
            end: hyperwall.end.into(),
            color: hyperwall.color,
        }
    }
}

impl PoncaireWall{
    fn find_circle_through_points(&self) -> (f64, f64, f64) {
        //https://math.stackexchange.com/questions/1503466/algebraic-solutions-for-poincar%C3%A9-disk-arcs
        let p = self.beginning.0;
        let q = self.end.0;

        let (px, py): (f64, f64) = (p[0], p[1]);
        let (qx, qy): (f64, f64) = (q[0], q[1]);

        let (px2, py2) = (px.powi(2), py.powi(2));
        let (qx2, qy2) = (px.powi(2), py.powi(2));

        //circle center
        let x0 = 
            (
                qy*(px2+py2+1.) - 
                py*(qx2+qy2+1.)
            ) / 
            (
                2.*(px*qy-py*qx)
            );
        let y0 = 
            (
                - qy*(px2+py2+1.) + 
                py*(qx2+qy2+1.)
            ) / 
            (
                2.*(px*qy-py*qx)
            );

        let x02 = x0.powi(2);
        let y02 = y0.powi(2);

        //circle radius
        let r0 = (x02+y02-1.).sqrt();
        
        (x0, y0, r0)
    }

    pub fn intersection(&self, angle: f64) -> Option<f64> {
        todo!()
    }
}
