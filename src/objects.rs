use std::ops::Range;

use rayon::prelude::*;

use crate::vec3::*;
use crate::hittable::{ Hittable, Hit };
use crate::material::Material;
use crate::ray::Ray;

#[derive(Debug, Clone)]
pub struct Sphere<Mat> {
    pub center: Point3,
    pub radius: f64,
    pub material: Mat,
}

impl<Mat> Sphere<Mat> {
    pub fn new(center: Point3, radius: f64, material: Mat) -> Self {
        Self { center, radius, material }
    }
}

impl<Mat: Material> Hittable for Sphere<Mat> {
    fn hit(&self, ray: &Ray, bounds: Range<f64>) -> Option<Hit> {
        let oc = ray.origin - self.center;
        let a = ray.dir.mag_sq();
        let half_b = oc.dot(&ray.dir);
        let c = oc.mag_sq() - self.radius * self.radius;
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
            let outward_normal = (hit_point - self.center).unit();

            let normal = if ray.dir.dot(&outward_normal) < 0.0 {
                outward_normal
            } else {
                -outward_normal
            };

            let scatter = self.material.scatter(ray, normal, hit_point);
            Some(Hit::new(hit_point, normal, t, scatter))
        } else {
            None
        }
    }
}

pub type BoxHittable = Box<dyn Hittable + Send + Sync>;

#[derive(Default)]
pub struct HitList {
    pub objects: Vec<BoxHittable>,
}

impl HitList {
    #[inline]
    pub fn new(objects: Vec<BoxHittable>) -> HitList {
        HitList { objects }
    }

    pub fn empty() -> HitList {
        HitList::default()
    }

    pub fn add(&mut self, object: impl Hittable + Send + Sync + 'static) {
        self.objects.push(Box::new(object));
    }
}

impl Hittable for HitList {
    fn hit(&self, ray: &Ray, bounds: Range<f64>) -> Option<Hit> {
        self.objects
            .iter()
            .filter_map(|hittable| hittable.hit(ray, bounds.clone()))
            .min_by(|a, b| a.t.partial_cmp(&b.t).unwrap_or(std::cmp::Ordering::Greater))

        /*
        self.objects
            .par_iter()
            .filter_map(|hittable| hittable.hit(ray, bounds.clone()))
            .min_by(|a, b| a.t.partial_cmp(&b.t).unwrap_or(std::cmp::Ordering::Greater))
        */
    }
}

#[derive(Default)]
pub struct WorldBuilder {
    objects: Vec<BoxHittable>,
}

impl WorldBuilder {
    pub fn build(&mut self) -> HitList {
        HitList::new(std::mem::take(&mut self.objects))
    }

    pub fn add(&mut self, object: impl Hittable + Send + Sync + 'static) -> &mut Self {
        self.objects.push(Box::new(object));
        self
    }
}
