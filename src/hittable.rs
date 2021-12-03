use std::ops::Range;
use std::sync::Arc;
use rayon::prelude::*;

use crate::utils::{ Point3, Vec3 };
use crate::ray::Ray;
use crate::material::Scatter;

pub trait Hittable {
    fn hit(&self, ray: &Ray, bounds: Range<f32>) -> Option<Hit>;
}

impl<'a, T: Hittable> Hittable for &'a T {
    #[inline]
    fn hit(&self, ray: &Ray, bounds: Range<f32>) -> Option<Hit> {
        (*self).hit(ray, bounds)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Hit {
    pub point: Point3,
    pub normal: Vec3,
    pub t: f32,
    pub scatter: Option<Scatter>,
}

impl Hit {
    pub fn new(point: Point3, normal: Vec3, t: f32, scatter: Option<Scatter>) -> Hit {
        Hit { point, normal, t, scatter }
    }
}
