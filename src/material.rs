use std::default::Default;

use rand::{ random, thread_rng };
use nalgebra_glm::vec3;

use crate::ray::Ray;
use crate::utils::{ self, Color, Vec3, Point3, color };

pub trait Material {
    fn scatter(&self, ray: &Ray, normal: Vec3, is_front: bool) -> Option<Scatter>;
}

impl<'a, Mat: Material> Material for &'a Mat {
    #[inline]
    fn scatter(&self, ray: &Ray, normal: Vec3, is_front: bool) -> Option<Scatter> {
        Mat::scatter(*self, ray, normal, is_front)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Scatter {
    pub attenuation: Color,
    pub scattered: Vec3,
}

impl Scatter {
    pub fn new(attenuation: Color, scattered: Vec3) -> Self {
        Self {
            attenuation,
            scattered,
        }
    }
}


#[derive(Debug, Clone)]
pub struct Diffuse {
    pub albedo: Color,
}

impl Diffuse {
    pub fn new(albedo: Color) -> Self {
        Diffuse { albedo }
    }
}

impl Material for Diffuse {
    fn scatter(&self, _: &Ray, normal: Vec3, _: bool) -> Option<Scatter> {
        let mut scatter_dir = normal + utils::random_unit();

        if (0.0..1e-8).contains(&scatter_dir.magnitude_squared()) {
            scatter_dir = normal;
        }

        Some(Scatter::new(self.albedo, scatter_dir))
    }
}

impl Default for Diffuse {
    fn default() -> Diffuse {
        Diffuse::new(color::mid_gray())
    }
}


#[derive(Debug, Clone)]
pub struct Metal {
    pub albedo: Color,
    pub fuzzy: f32,
}

impl Metal {
    pub fn new(albedo: Color, fuzzy: f32) -> Self {
        Self { albedo, fuzzy }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, normal: Vec3,  _: bool) -> Option<Scatter> {
        let reflected = reflect(ray.dir, normal) + utils::random_unit() * self.fuzzy;

        if reflected.dot(&normal) >= 0.0 {
            Some(Scatter::new(self.albedo, reflected))
        } else {
            None
        }
    }
}

impl Default for Metal {
    fn default() -> Metal {
        Metal::new(color::white(), 0.0)
    }
}

#[derive(Debug, Clone)]
pub struct Dielectric {
    pub ior: f32,
}

impl Dielectric {
    pub fn new(ior: f32) -> Self {
        Self { ior }
    }

    // Use Schlick's approximation for reflectance.
    fn reflectance(cos: f32, ior_ratio: f32) -> f32 {
        let r0 = ((1.0 - ior_ratio) / (1.0 + ior_ratio)).powi(2);
        r0 + (1.0 - r0) * (1.0 - cos).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, normal: Vec3, is_front: bool) -> Option<Scatter> {
        let ior_ratio = if is_front { 1.0 / self.ior } else { self.ior };

        let cos_theta = (-ray.dir).dot(&normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        // Total Internal Reflection
        let tir = ior_ratio * sin_theta > 1.0;
        let reflectance = Dielectric::reflectance(cos_theta, ior_ratio);

        let scattered = if tir || reflectance > random::<f32>() {
            reflect(ray.dir.normalize(), normal)
        } else {
            refract(ray.dir.normalize(), normal, ior_ratio)
        };

        Some(Scatter::new(color::white(), scattered))
    }
}

// This struct exists in order to avoid boxing.
#[derive(Debug, Clone)]
pub enum CommonMat {
    Diffuse(Diffuse),
    Metal(Metal),
    Dielectric(Dielectric),
}

impl Material for CommonMat {
    fn scatter(&self, ray: &Ray, normal: Vec3, is_front: bool) -> Option<Scatter> {
        use CommonMat::*;

        match self {
            Diffuse(mat)     => mat.scatter(ray, normal, is_front),
            Metal(mat)       => mat.scatter(ray, normal, is_front),
            Dielectric(mat) => mat.scatter(ray, normal, is_front),
        }
    }
}

impl From<Diffuse> for CommonMat {
    fn from(v: Diffuse) -> CommonMat {
        CommonMat::Diffuse(v)
    }
}

impl From<Metal> for CommonMat {
    fn from(v: Metal) -> CommonMat {
        CommonMat::Metal(v)
    }
}

impl From<Dielectric> for CommonMat {
    fn from(v: Dielectric) -> CommonMat {
        CommonMat::Dielectric(v)
    }
}

pub fn reflect(incident: Vec3, normal: Vec3) -> Vec3 {
    incident - normal * 2.0 * incident.dot(&normal)
}

fn refract(incident: Vec3, normal: Vec3, ior_ratio: f32) -> Vec3 {
    let cos_theta = (-incident).dot(&normal).min(1.0);
    let refracted_perp = (incident + normal * cos_theta) * ior_ratio;
    let refracted_par  = -normal * (1.0 - refracted_perp.magnitude_squared()).abs().sqrt();
    refracted_perp + refracted_par
}

