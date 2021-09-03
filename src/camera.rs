
use crate::vec3::{ Point3, Vec3 };
use crate::ray::Ray;

pub struct Camera {
    origin: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    lower_left_corner: Point3,
}

impl Camera {
    pub fn new(aspect_ratio: f64) -> Camera {
        let viewport_height = 2.0;
        let viewport_width  = aspect_ratio * viewport_height;
        let focal_length    = 1.0;

        let origin = Point3::new(0.0, 0.0, 0.0);
        let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
        let vertical = Vec3::new(0.0, viewport_height, 0.0);
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

        Camera { origin, horizontal, vertical, lower_left_corner }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        // The point at the viewport 
        let viewport_point = self.lower_left_corner + self.horizontal * u + self.vertical * v;
        Ray::new(self.origin, viewport_point - self.origin)
    }
}
