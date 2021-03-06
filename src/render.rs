use std::default::Default;
use std::sync::atomic::{ AtomicUsize, Ordering };
use std::convert::TryInto;

use rayon::prelude::*;
use rayon::iter;
use rand::random;

use crate::objects::{ BoxedHitList, Sphere };
use crate::material::CommonMat;
use crate::hittable::Hittable;
use crate::camera::Camera;
use crate::utils::{ self, color, Color };

pub struct Scene<T> {
    pub world: T,
    pub camera: Camera,
    pub config: Render,
}

impl<T: Hittable + Send + Sync> Scene<T> {
    pub fn new(world: T, camera: Camera, config: Render) -> Self {
        Self { world, camera, config }
    }
}

#[derive(Debug, Clone)]
pub struct Render {
    pub aspect_ratio: f32,
    pub width: usize,
    pub height: usize,
    pub samples_per_pixel: usize,
    pub max_bounces: usize,
}

impl Render {
    pub fn new(
        aspect_ratio: f32,
        width: usize,
        height: usize,
        samples_per_pixel: usize,
        max_bounces: usize,
    ) -> Self {
        Self {
            aspect_ratio,
            width,
            height,
            samples_per_pixel,
            max_bounces,
        }
    }

    pub fn with_ratio(aspect_ratio: f32, height: usize) -> Self {
        Self {
            aspect_ratio,
            width: (height as f32 * aspect_ratio).ceil() as usize,
            height,
            ..Self::default()
        }
    }
}

impl Default for Render {
    fn default() -> Self {
        Render {
            aspect_ratio: 16.0 / 9.0,
            width: 480,
            height: 854,
            samples_per_pixel: 10,
            max_bounces: 5,
        }
    }
}

pub struct RenderBuilder {
    render: Render,
}

impl RenderBuilder {
    pub fn new() -> Self {
        Self {
            render: Render::default(),
        }
    }

    pub fn build(&mut self) -> Render {
        self.render.clone()
    }

    pub fn with_ratio(&mut self, aspect_ratio: f32, height: usize) -> &mut Self {
        self.render.width = (height as f32 * aspect_ratio).ceil() as usize;
        self.render.height = height;
        self.render.aspect_ratio = aspect_ratio;
        self
    }

    pub fn with_samples(&mut self, samples_per_pixel: usize) -> &mut Self {
        self.render.samples_per_pixel = samples_per_pixel;
        self
    }

    pub fn with_max_bounces(&mut self, max_bounces: usize) -> &mut Self {
        self.render.max_bounces = max_bounces;
        self
    }

    pub fn with_dimensions(&mut self, width: usize, heigth: usize) -> &mut Self {
        self.render.width  = width;
        self.render.height = heigth;
        self.render.aspect_ratio = width as f32 / heigth as f32;
        self
    }
}

pub fn multi_thread_render<T: Hittable + Send + Sync>(scene: Scene<T>) {
    let Scene { world, camera, config } = scene;
    let Render {
        aspect_ratio: _,
        width,
        height,
        samples_per_pixel,
        max_bounces,
    } = config;

    let width = width as u32;
    let height = height as u32;

    let count = AtomicUsize::new(0);

    let mut img = image::RgbImage::new(width as u32, height as u32);

    img
        .par_chunks_exact_mut(3)
        .enumerate()
        .for_each(|(i, pixel)| {
            let y = i as u32 / width;
            let x = i as u32 % width;

            // Invert the y coordinate so higher of y go up.
            let y = height - y as u32;

            let pixel = rgb_mut_ref(pixel.try_into().unwrap());

            let mut color = color::black();
            for _ in 0..samples_per_pixel {
                let u = (x as f32 + random::<f32>()) / (width  as f32 - 1.0);
                let v = (y as f32 + random::<f32>()) / (height as f32 - 1.0);

                color += camera.get_ray(u, v).compute_color(&world, max_bounces);
            }

            let pixel_val = color / (samples_per_pixel as f32);
            *pixel = utils::to_rgb(nalgebra_glm::sqrt(&pixel_val));

            let oldval = count.fetch_add(1, Ordering::SeqCst);

            if oldval % 60 == 0 {
                let percent = (oldval as f32 * 100.0) / (width * height as u32) as f32;
                eprint!("\r[{:03.0}%] Rendering", percent);
            }
        });

    let stdout = std::io::stdout();
    let mut stdout = stdout.lock();
    let mut encoder = image::codecs::jpeg::JpegEncoder::new_with_quality(&mut stdout, 100);

    encoder.encode_image(&img).unwrap();

    eprintln!("\nDone!");
}

pub fn simple_multi_thread_render<T: Hittable + Send + Sync>(scene: Scene<T>) {
    let Scene { world, camera, config } = scene;
    let Render {
        aspect_ratio: _,
        width,
        height,
        samples_per_pixel,
        max_bounces,
    } = config;

    let width = width as u32;
    let height = height as u32;
    let mut img = image::RgbImage::new(width as u32, height as u32);

    let count = AtomicUsize::new(0);

    let render_row = |y, row: &mut [u8]| {
        // Invert the y coordinate so higher of y go up.
        let y = height - y as u32;

        let row_iter = row
            .as_chunks_mut().0 // &mut [[u8; 3]]
            .iter_mut()        // impl Iterator<Item = &mut [u8; 3]>
            .map(rgb_mut_ref); // impl Iterator<Item = &mut Rgb<u8>>

        for (x, pixel) in row_iter.enumerate() {
            let mut color = color::black();
            for _ in 0..samples_per_pixel {
                let u = (x as f32 + random::<f32>()) / (width  as f32 - 1.0);
                let v = (y as f32 + random::<f32>()) / (height as f32 - 1.0);

                color += camera.get_ray(u, v).compute_color(&world, max_bounces);
            }

            let pixel_val = color / (samples_per_pixel as f32);
            *pixel = utils::to_rgb(nalgebra_glm::sqrt(&pixel_val));

            let oldval = count.fetch_add(1, Ordering::SeqCst);
            if oldval % 60 == 0 {
                let percent = (oldval as f32 * 100.0) / (width * height) as f32;
                eprint!("\r[{:03.0}%] Rendering", percent);
            }
        }
    };

    img
        .par_chunks_exact_mut(width as usize * 3)
        .enumerate()
        .for_each(|(y, row)| render_row(y, row));

    let stdout = std::io::stdout();
    let mut stdout = stdout.lock();
    let mut encoder = image::codecs::jpeg::JpegEncoder::new_with_quality(&mut stdout, 100);

    encoder.encode_image(&img).unwrap();

    eprintln!("\nDone!");
}

pub fn single_thread_render<T: Hittable>(scene: Scene<T>) {
    let Scene { world, camera, config } = scene;
    let Render {
        aspect_ratio: _,
        width,
        height,
        samples_per_pixel,
        max_bounces,
    } = config;

    let width = width as u32;
    let height = height as u32;
    let mut img = image::RgbImage::new(width as u32, height as u32);

    let stdout = std::io::stdout();
    let mut stdout = stdout.lock();
    let mut encoder = image::codecs::jpeg::JpegEncoder::new_with_quality(&mut stdout, 100);

    encoder.encode_image(&img).unwrap();

    let mut count = 0;
    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let y = height - y;
        let mut color = color::black();

        for _ in 0..samples_per_pixel {
            let u = (x as f32 + random::<f32>()) / (width  as f32 - 1.0);
            let v = (y as f32 + random::<f32>()) / (height as f32 - 1.0);
            color += camera.get_ray(u, v).compute_color(&world, max_bounces)
        }

        let pixel_val = color / (samples_per_pixel as f32);
        *pixel = utils::to_rgb(nalgebra_glm::sqrt(&pixel_val));

        count += 1;
        let percent = (count as f32 * 100.0) / (width * height) as f32;
        eprint!("\r[{:03.0}%] Rendering", percent);
    }

    eprintln!("\nDone!");
}

fn rgb_mut_ref<T: image::Primitive>(data: &mut [T; 3]) -> &mut image::Rgb<T> {
    // Safety: image::Rgb is repr(C) so it is transparent to the underlying data.
    unsafe {
        std::mem::transmute(data)
    }
}

pub fn random_scene() -> Vec<Sphere<CommonMat>> {
    use crate::objects::{ WorldBuilder, Sphere };
    use crate::material::{ Dielectric, Diffuse, Metal };
    use crate::utils::{ Vec3, Point3 };

    let mut world_builder = WorldBuilder::default();

    let ground_material = Diffuse::new(Color::new(0.5, 0.5, 0.5));
    world_builder.add(Sphere::new(Point3::new(0.0, -1000.0, 0.0), 1000.0, ground_material.into()));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random::<f32>();
            let center = nalgebra_glm::vec3(
                a as f32 + 0.9 * random::<f32>(),
                0.2,
                b as f32 + 0.9 * random::<f32>(),
            );

            if (center - nalgebra_glm::vec3(4.0, 0.2, 0.0)).magnitude() > 0.9 {
                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = color::random().component_mul(&color::random());
                    let sphere_material = Diffuse::new(albedo);
                    world_builder.add(Sphere::new(center, 0.2, sphere_material.into()));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = color::random();
                    let fuzz = random::<f32>() * 0.5;
                    let sphere_material = Metal::new(albedo, fuzz);
                    world_builder.add(Sphere::new(center, 0.2, sphere_material.into()));
                } else {
                    // glass
                    let sphere_material = Dielectric::new(1.5);
                    world_builder.add(Sphere::new(center, 0.2, sphere_material.into()));
                }
            }
        }
    }

    let material1 = Dielectric::new(1.5);
    world_builder.add(Sphere::new(Point3::new(0.0, 1.0, 0.0), 1.0, material1.into()));

    let material2 = Diffuse::new(Color::new(0.4, 0.2, 0.1));
    world_builder.add(Sphere::new(Point3::new(-4.0, 1.0, 0.0), 1.0, material2.into()));

    let material3 = Metal::new(Color::new(0.7, 0.6, 0.5), 0.0);
    world_builder.add(Sphere::new(Point3::new(4.0, 1.0, 0.0), 1.0, material3.into()));

    return world_builder.build();
}
