
use crate::utils::{ self, Point3, Vec3 };
use crate::ray::Ray;

pub struct Camera {
    origin: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    lower_left_corner: Point3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    lens_radius: f32,
}

impl Camera {
    pub fn new(
        look_from: Vec3,
        look_at: Vec3,
        vup: Vec3,
        vert_fov: f32,
        aspect_ratio: f32,
        aperture: f32,
        focus_dist: f32,
    ) -> Camera {

        let theta = vert_fov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (look_from - look_at).normalize();
        let u = vup.cross(&w).normalize();
        let v = w.cross(&u);

        let origin = look_from;
        let horizontal = u * viewport_width * focus_dist;
        let vertical = v * viewport_height * focus_dist;
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - w * focus_dist;

        let lens_radius = aperture / 2.0;

        Camera {
            origin,
            horizontal,
            vertical,
            lower_left_corner,
            u,
            v,
            w,
            lens_radius,
        }
    }

    pub fn get_ray(&self, s: f32, t: f32) -> Ray {
        let rand = utils::random_in_unit_disc() * self.lens_radius;
        let offset = self.u * rand.x + self.v * rand.y;
        // The point at the viewport
        let viewport_point = self.lower_left_corner + self.horizontal * s + self.vertical * t;
        Ray::new(self.origin + offset, viewport_point - self.origin - offset)
    }
}
