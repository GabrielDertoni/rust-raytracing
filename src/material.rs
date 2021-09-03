use std::default::Default;

use rand::thread_rng;

use crate::ray::Ray;
use crate::vec3::{ Color, Vec3, Point3 };

pub trait Material {
    fn scatter(&self, ray: &Ray, normal: Vec3, point: Point3) -> Option<Scatter>;
}

#[derive(Debug, Clone, Copy)]
pub struct Scatter {
    pub attenuation: Color,
    pub scatter: Ray,
}

impl Scatter {
    pub fn new(attenuation: Color, scatter: Ray) -> Self {
        Self {
            attenuation,
            scatter,
        }
    }
}


#[derive(Debug, Clone)]
pub struct Diffuse {
    albedo: Color,
}

impl Diffuse {
    pub fn new(albedo: Color) -> Self {
        Diffuse { albedo }
    }
}

impl Material for Diffuse {
    fn scatter(&self, _: &Ray, normal: Vec3, point: Point3) -> Option<Scatter> {
        let mut rng = thread_rng();
        let mut scatter_dir = normal + Vec3::<f64>::random_unit(&mut rng);

        if (0.0..1e-8).contains(&scatter_dir.mag_sq()) {
            scatter_dir = normal;
        }

        let scattered = Ray::new(point, scatter_dir);
        Some(Scatter::new(self.albedo, scattered))
    }
}

impl Default for Diffuse {
    fn default() -> Diffuse {
        Diffuse::new(Color::mid_gray())
    }
}


#[derive(Debug, Clone)]
pub struct Metal {
    albedo: Color,
    fuzzy: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzzy: f64) -> Self {
        Self { albedo, fuzzy }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, normal: Vec3, point: Point3) -> Option<Scatter> {
        let mut rng = thread_rng();
        let reflected = ray.dir.reflect(normal) + Vec3::<f64>::random_unit(&mut rng) * self.fuzzy;
        let scattered = Ray::new(point, reflected);

        if scattered.dir.dot(&normal) >= 0.0 {
            Some(Scatter::new(self.albedo, scattered))
        } else {
            None
        }
    }
}

impl Default for Metal {
    fn default() -> Metal {
        Metal::new(Color::white(), 0.0)
    }
}


