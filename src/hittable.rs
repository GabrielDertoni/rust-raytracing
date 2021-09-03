use std::ops::Range;
use std::sync::Arc;
use rayon::prelude::*;

use crate::vec3::{ Point3, Vec3 };
use crate::ray::Ray;
use crate::material::Scatter;

pub trait Hittable {
    fn hit(&self, ray: &Ray, bounds: Range<f64>) -> Option<Hit>;
}

impl<'a, T: Hittable> Hittable for &'a T {
    #[inline]
    fn hit(&self, ray: &Ray, bounds: Range<f64>) -> Option<Hit> {
        (*self).hit(ray, bounds)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Hit {
    pub point: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub scatter: Option<Scatter>,
}

impl Hit {
    pub fn new(point: Point3, normal: Vec3, t: f64, scatter: Option<Scatter>) -> Hit {
        Hit { point, normal, t, scatter }
    }
}


/*
pub fn raymarch(hittable: impl Hittable, ray: &Ray, bounds: Range<f64>) -> Option<Hit> {
    let mut curr_point = ray.origin;
    let mut t = 0.0;

    loop {
        let dist = hittable.dist_to_point(ray.at(t));

        if dist < 1e-3 && bounds.start < t {
            return None;
        }

        t += dist;
    }
}
*/
