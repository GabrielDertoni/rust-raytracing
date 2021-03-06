use std::ops::Range;

use rayon::prelude::*;

use crate::utils::{ Vec3, Point3, Color };
use crate::hittable::{ Hittable, Hit };
use crate::material::Material;
use crate::ray::Ray;

#[derive(Debug, Clone)]
pub struct Sphere<Mat> {
    pub center: Point3,
    pub radius: f32,
    pub material: Mat,
}

impl<Mat> Sphere<Mat> {
    pub fn new(center: Point3, radius: f32, material: Mat) -> Self {
        Self { center, radius, material }
    }
}

impl<Mat: Material> Hittable for Sphere<Mat> {
    fn hit(&self, ray: &Ray, bounds: Range<f32>) -> Option<Hit> {
        let oc = ray.origin - self.center;
        let a = ray.dir.magnitude_squared();
        let half_b = oc.dot(&ray.dir);
        let c = oc.magnitude_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant > 0.0 {
            let disc_sqrt = discriminant.sqrt();
            let mut t = (-half_b - disc_sqrt) / a;

            if !bounds.contains(&t) {
                t = (-half_b + disc_sqrt) / a;

                if !bounds.contains(&t) {
                    return None;
                }
            }

            let hit_point = ray.at(t);
            let outward_normal = (hit_point - self.center) / self.radius;

            let (normal, is_front) = if ray.dir.dot(&outward_normal) < 0.0 {
                (outward_normal, true)
            } else {
                (-outward_normal, false)
            };

            let scatter = self.material.scatter(ray, normal, is_front);
            Some(Hit::new(hit_point, normal, t, scatter))
        } else {
            None
        }
    }
}

pub type BoxHittable = Box<dyn Hittable + Send + Sync>;

impl Hittable for Box<dyn Hittable + Send + Sync> {
    #[inline]
    fn hit(&self, ray: &Ray, bounds: Range<f32>) -> Option<Hit> {
        self.as_ref().hit(ray, bounds)
    }
}

impl<T: Hittable> Hittable for Vec<T> {
    fn hit(&self, ray: &Ray, bounds: Range<f32>) -> Option<Hit> {
        self.iter()
            .filter_map(|hittable| hittable.hit(ray, bounds.clone()))
            .min_by(|a, b| a.t.partial_cmp(&b.t).unwrap_or(std::cmp::Ordering::Greater))
    }
}

#[derive(Default)]
pub struct BoxedHitList {
    pub objects: Vec<BoxHittable>,
}

impl BoxedHitList {
    #[inline]
    pub fn new(objects: Vec<BoxHittable>) -> BoxedHitList {
        BoxedHitList { objects }
    }

    pub fn empty() -> BoxedHitList {
        BoxedHitList::default()
    }

    pub fn add(&mut self, object: impl Hittable + Send + Sync + 'static) {
        self.objects.push(Box::new(object));
    }
}

impl Hittable for BoxedHitList {
    #[inline]
    fn hit(&self, ray: &Ray, bounds: Range<f32>) -> Option<Hit> {
        self.objects.hit(ray, bounds)
    }
}

pub struct WorldBuilder<T> {
    objects: Vec<T>,
}

impl<T> WorldBuilder<T> {
    pub fn build(&mut self) -> Vec<T> {
        std::mem::take(&mut self.objects)
    }

    pub fn add(&mut self, object: T) -> &mut Self {
        self.objects.push(object);
        self
    }
}

impl WorldBuilder<Box<dyn Hittable + Send + Sync + 'static>> {
    pub fn into_boxed_list(&mut self) -> BoxedHitList {
        BoxedHitList::new(std::mem::take(&mut self.objects))
    }

    pub fn add_boxed(&mut self, object: impl Hittable + Send + Sync + 'static) -> &mut Self {
        self.objects.push(Box::new(object));
        self
    }
}

impl<T> Default for WorldBuilder<T> {
    fn default() -> WorldBuilder<T> {
        WorldBuilder { objects: Vec::new() }
    }
}
