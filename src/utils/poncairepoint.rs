use std::cmp::Ordering;

use hyperpoint::{HyperWall, Hyperpoint};
use nalgebra::*;
use point::Point;
use serde::Deserialize;

use crate::utils::hyperpoint;

use super::{color::RGBColor, point};

/// Struct representing a point on the 
/// Poincare disk model.
/// Wrapper for nalgebra's Point2.
#[derive(Clone, Debug, Deserialize)]
pub struct PoincarePoint(pub Point2<f64>);

impl From<Hyperpoint> for PoincarePoint {
    fn from(hyperpoint: Hyperpoint) -> Self {
        let denom = hyperpoint.0[2] + 1.0;
        PoincarePoint::new(hyperpoint.0[0] / denom, hyperpoint.0[1] / denom)
    }
}

impl PoincarePoint {
    pub fn new(x: f64, y: f64) -> PoincarePoint {
        PoincarePoint {
            0: Point2::<f64>::new(x, y),
        }
    }
}

impl point::Point for PoincarePoint {
    /// Return the Minkowski inner product of the two vectors provided, where the
    /// last co-ordinate is interpreted as being time-like.
    fn minkowski_dot(a: &PoincarePoint, b: &PoincarePoint) -> f64 {
        a.0[0] * b.0[0] - a.0[1] * b.0[1]
    }

    /// Distance to origin in the Poincare metric.
    fn distance_to_origin(&self) -> f64 {
        self.distance_to(&PoincarePoint::new_at_origin())
    }

    /*fn distance_to_origin(&self) -> f64 {
        let euclidian_distance =
            (self.0[0].powi(2) + self.0[1].powi(2))
            .sqrt();
        euclidian_distance
    }*/

    /// New point at 0, 0.
    fn new_at_origin() -> Self {
        PoincarePoint::new(0., 0.)
    }

    /// Distance to another point in the Poincare metric.
    fn distance_to(&self, to: &Self) -> f64 {
        let (x1, y1): (f64, f64) = (self.0[0], self.0[1]);
        let (x2, y2): (f64, f64) = (to.0[0], to.0[1]);

        let z1 = nalgebra::Complex::new(x1, y1);
        let z2 = nalgebra::Complex::new(x2, y2);
        let one = nalgebra::Complex::new(1., 0.);

        let upper: Complex<f64> = z1 - z2;
        let lower: Complex<f64> = one - z1 * (z2.conj());
        let div: Complex<f64> = upper / lower;
        let norm: f64 = div.norm();
        let result: f64 = 2. * norm.atanh();
        result
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct PoincareWall {
    pub beginning: PoincarePoint,
    pub end: PoincarePoint,
    pub color: RGBColor,
}

impl From<HyperWall> for PoincareWall {
    fn from(hyperwall: HyperWall) -> PoincareWall {
        PoincareWall {
            beginning: hyperwall.beginning.into(),
            end: hyperwall.end.into(),
            color: hyperwall.color,
        }
    }
}

impl PoincareWall {
    /// Constructs a geodesic between wall ends in the Poncarie disk model.
    /// Returns: a triplet (x_center, y_center, radius)
    fn find_circle_through_points(&self) -> (f64, f64, f64) {
        //https://math.stackexchange.com/questions/1503466/algebraic-solutions-for-poincar%C3%A9-disk-arcs
        let p = self.beginning.0;
        let q = self.end.0;

        let (px, py): (f64, f64) = (p[0], p[1]);
        let (qx, qy): (f64, f64) = (q[0], q[1]);

        let (px2, py2) = (px.powi(2), py.powi(2));
        let (qx2, qy2) = (px.powi(2), py.powi(2));

        //circle center
        let x0 = (qy * (px2 + py2 + 1.) - py * (qx2 + qy2 + 1.)) / (2. * (px * qy - py * qx));
        let y0 = (-qx * (px2 + py2 + 1.) + px * (qx2 + qy2 + 1.)) / (2. * (px * qy - py * qx));

        let x02 = x0.powi(2);
        let y02 = y0.powi(2);

        //circle radius
        let r0 = (x02 + y02 - 1.).sqrt();

        (x0, y0, r0)
    }

    /// Returns the angle of a point on the walls geodesic circle.
    /// Angle calculated relative to the X axis, eg.
    /// assuming center point of x: 0, y: 0
    /// a point at x: 0, y: -1 will have angle: 1.5PI
    fn find_angle_on_wall(&self, x: f64, y: f64, x0: f64, y0: f64) -> f64 {
        let ang = (y - y0).atan2(x - x0);
        Self::normalize_angle(ang)
    }

    /// Adds 2*PI to negative angles, so they are positive
    fn normalize_angle(angle: f64) -> f64 {
        match angle < 0. {
            true => angle + 2. * std::f64::consts::PI,
            false => angle,
        }
    }

    /// Checks whether p is located within the arc inscribed by endpoints of
    /// the wall on the geodesic circle.
    fn is_point_on_wall(&self, p: PoincarePoint, x0: f64, y0: f64) -> bool {
        let (x1, y1): (f64, f64) = (self.beginning.0[0], self.beginning.0[1]);
        let (x2, y2): (f64, f64) = (self.end.0[0], self.end.0[1]);
        let (xp, yp): (f64, f64) = (p.0[0], p.0[1]);

        let angle1 = self.find_angle_on_wall(x1, y1, x0, y0);
        let angle2 = self.find_angle_on_wall(x2, y2, x0, y0);
        let anglep = self.find_angle_on_wall(xp, yp, x0, y0);

        let (anglemin, anglemax) = (angle1.min(angle2), angle1.max(angle2));
        let result;
        if anglep < anglemin || anglep > anglemax {
            result = false;
        } else {
            result = true;
        }
        //println!("min:{},max:{},p:{},{:?}",anglemin, anglemax, anglep, result);
        result
    }

    /// Finds distance from origin to the closest intersection point, if that point lies on the wall.
    /// Uses Poincare metric implemented on Point struct.
    pub fn find_distance_of_intersection_with_ray(&self, angle: f64) -> Option<f64> {
        let (a, b, r) = self.find_circle_through_points();
        let m = angle; //.tan();
        let r2 = r.powi(2);
        let m2 = m.powi(2);

        let delta = r2 * (1. + m2) - (b - m * a).powi(2);
        let deltasqrt = delta.sqrt();
        if deltasqrt.is_nan() {
            return None;
        }

        let x1 = (a + b * m + deltasqrt) / (1. + m2);
        let x2 = (a + b * m - deltasqrt) / (1. + m2);

        let y1 = (a * m + b * m2 + m * deltasqrt) / (1. + m2);
        let y2 = (a * m + b * m2 - m * deltasqrt) / (1. + m2);

        let p1 = PoincarePoint::new(x1, y1);
        let p2 = PoincarePoint::new(x2, y2);

        let d1 = p1.distance_to_origin();
        let d2 = p2.distance_to_origin();

        let mut points = vec![(d1, p1), (d2, p2)];
        points.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(Ordering::Equal));

        let (closest_distance, closest_point) = points.into_iter().find(|e| {
            let x = (e.1).0[0];
            x > 0. && !e.0.is_nan()
        })?;

        match self.is_point_on_wall(closest_point, a, b) {
            true => Some(closest_distance),
            false => None,
        }
    }
}
