use rand::{ thread_rng, Rng };

use crate::utils::{ self, Color, Vec3, Point3, color };
use crate::hittable::{ Hittable, Hit };

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub dir: Vec3,
    pub origin: Point3,
}

impl Ray {
    /// Create a new ray.
    pub fn new(origin: Point3, dir: Vec3) -> Ray {
        Ray { dir, origin }
    }

    /// Get a reference to the ray's dir.
    pub fn dir(&self) -> &Vec3 {
        &self.dir
    }

    /// Get a reference to the ray's origin.
    pub fn origin(&self) -> &Point3 {
        &self.origin
    }

    pub fn at(&self, t: f32) -> Point3 {
        self.origin + self.dir * t
    }

    pub fn compute_color(&self, world: impl Hittable, max_depth: usize) -> Color {
        let mut ray = *self;
        let mut color = Color::new(1., 1., 1.);
        for _ in 0..max_depth {
            match world.hit(&ray, 0.001..f32::INFINITY) {
                None => {
                    color.component_mul_assign(&self.bg_color());
                    break;
                }

                Some(Hit { scatter: None, .. }) => {
                    color.component_mul_assign(&color::black());
                    break;
                }

                Some(Hit { scatter: Some(s), point, .. }) => {
                    ray = Ray::new(point, s.scattered);
                    color.component_mul_assign(&s.attenuation);
                }
            }
        }
        color
    }

    pub fn bg_color(&self) -> Color {
        let dir = self.dir.normalize();
        let t = dir.y / 2.0 + 0.5;
        color::lerp(nalgebra_glm::vec3(1.0, 1.0, 1.0), nalgebra_glm::vec3(0.5, 0.7, 1.0), t)
    }
}
