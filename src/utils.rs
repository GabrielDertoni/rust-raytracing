use rand::random;

pub use nalgebra_glm::Vec3;

pub type Color = nalgebra_glm::Vec3;
pub type Point3 = nalgebra_glm::Vec3;


pub fn random_in_unit_disc() -> Vec3 {
    let theta = random::<f32>() * std::f32::consts::TAU;
    let rho   = random::<f32>();
    nalgebra_glm::vec3(
        rho * theta.cos(),
        rho * theta.sin(),
        0.0
    )
}

pub fn random_in_unit_sphere() -> Vec3 {
    let phi   = random::<f32>() * std::f32::consts::PI;
    let theta = random::<f32>() * std::f32::consts::TAU;
    let rho   = random::<f32>();
    nalgebra_glm::vec3(
        rho * phi.sin() * theta.cos(),
        rho * phi.sin() * theta.sin(),
        rho * phi.cos(),
    )
}

pub fn random_unit() -> Vec3 {
    random_in_unit_sphere().normalize()
}

pub fn to_rgb(color: Color) -> image::Rgb<u8> {
    let bytes = nalgebra_glm::try_convert(
        nalgebra_glm::clamp(&color, 0.0, 0.999) * 256.0
    ).unwrap_or(nalgebra_glm::vec3(255, 255, 255));
    image::Rgb([bytes[0], bytes[1], bytes[2]])
}

pub mod color {
    use super::*;

    #[inline]
    pub fn white() -> Color {
        nalgebra_glm::vec3(1.0, 1.0, 1.0)
    }

    #[inline]
    pub fn black() -> Color {
        nalgebra_glm::vec3(0.0, 0.0, 0.0)
    }

    #[inline]
    pub fn mid_gray() -> Color {
        nalgebra_glm::vec3(0.5, 0.5, 0.5)
    }

    #[inline]
    pub fn red() -> Color {
        nalgebra_glm::vec3(1.0, 0.0, 0.0)
    }

    #[inline]
    pub fn green() -> Color {
        nalgebra_glm::vec3(0.0, 1.0, 0.0)
    }

    #[inline]
    pub fn blue() -> Color {
        nalgebra_glm::vec3(0.0, 0.0, 1.0)
    }

    #[inline]
    pub fn lerp(start: Color, end: Color, step: f32) -> Color {
        start + (end - start) * step
    }

    #[inline]
    pub fn random() -> Color {
        nalgebra_glm::vec3(rand::random(), rand::random(), rand::random())
    }

    #[inline]
    pub fn new(r: f32, g: f32, b: f32) -> Color {
        nalgebra_glm::vec3(r, g, b)
    }
}
