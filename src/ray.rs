use rand::{ thread_rng, Rng };

use crate::vec3::*;
use crate::hittable::Hittable;

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

    pub fn at(&self, t: f64) -> Point3 {
        self.origin + self.dir * t
    }

    pub fn compute_color(&self, world: impl Hittable, max_depth: usize) -> Color {
        if let Some(hit) = world.hit(self, 0.001..f64::INFINITY) {
            match (hit.scatter, max_depth) {
                (_      , 0) |
                (None   , _) => Color::black(),
                (Some(s), _) => {
                    let ray = Ray::new(hit.point, s.scattered);
                    ray.compute_color(world, max_depth - 1) * s.attenuation
                }
            }
        } else {
            self.bg_color()
        }
    }

    pub fn bg_color(&self) -> Color {
        let dir = self.dir.unit();
        let t = dir.y / 2.0 + 0.5;
        Color::lerp(Color::new(1.0, 1.0, 1.0), Color::new(0.5, 0.7, 1.0), t)
    }
}
