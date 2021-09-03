use std::default::Default;
use std::sync::atomic::{ AtomicUsize, Ordering };

use rayon::prelude::*;
use rayon::iter;
use rand::{ thread_rng, Rng };

use crate::objects::HitList;
use crate::camera::Camera;
use crate::vec3::Color;

pub struct Scene {
    pub world: HitList,
    pub camera: Camera,
    pub config: Render,
}

impl Scene {
    pub fn new(world: HitList, camera: Camera, config: Render) -> Self {
        Self { world, camera, config }
    }
}

#[derive(Debug, Clone)]
pub struct Render {
    pub aspect_ratio: f64,
    pub width: usize,
    pub height: usize,
    pub samples_per_pixel: usize,
    pub max_bounces: usize,
}

impl Render {
    pub fn new(
        aspect_ratio: f64,
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

    pub fn with_ratio(aspect_ratio: f64, height: usize) -> Self {
        Self {
            aspect_ratio,
            width: (height as f64 * aspect_ratio).ceil() as usize,
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

    pub fn with_ratio(&mut self, aspect_ratio: f64, height: usize) -> &mut Self {
        self.render.width = (height as f64 * aspect_ratio).ceil() as usize;
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
        self.render.aspect_ratio = width as f64 / heigth as f64;
        self
    }
}

pub fn multi_thread_render(scene: Scene) {
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

    let (tx, rx) = std::sync::mpsc::channel();

    rayon::scope(|s| {
        s.spawn(|_| {
            for el in img.enumerate_pixels_mut() {
                tx.send(el).unwrap();
            }
            drop(tx);
        });

        let (progress_sender, progress_receiver) = std::sync::mpsc::sync_channel(16);

        s.spawn(move |_| {
            rx.into_iter()
                .par_bridge()
                .for_each(|(x, y, pixel)| {
                    let color = (0..samples_per_pixel)
                        .into_par_iter()
                        .map(|_| {
                            let mut rng = thread_rng();
                            let u = (x as f64 + rng.gen_range(0.0..1.0)) / (width  as f64 - 1.0);
                            let v = ((height - y) as f64 + rng.gen_range(0.0..1.0)) / (height as f64 - 1.0);

                            camera.get_ray(u, v).compute_color(&world, max_bounces)
                        })
                        .reduce(Color::black, |a, b| a + b);

                    *pixel = (color / (samples_per_pixel as f64)).sqrt().into();
                    progress_sender.send(()).unwrap();
                });
            
        });

        s.spawn(move |_| {
            let mut count = 0;
            while let Ok(_) = progress_receiver.recv() {
                count += 1;
                let percent = (count as f64 * 100.0) / (width * height) as f64;
                eprint!("\r[{:03.0}%] Rendering", percent);
            }
        });
    });

    let stdout = std::io::stdout();
    let mut stdout = stdout.lock();
    let mut encoder = image::codecs::jpeg::JpegEncoder::new_with_quality(&mut stdout, 100);

    encoder.encode_image(&img).unwrap();

    eprintln!("\nDone!");
}

fn rgb_mut_ref<T: image::Primitive>(data: &mut [T; 3]) -> &mut image::Rgb<T> {
    // Safety: image::Rgb is repr(C) so it is transparent to the underlying data.
    unsafe {
        std::mem::transmute(data)
    }
}

pub fn simple_multi_thread_render(scene: Scene) {
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
        let mut rng = thread_rng();

        // Invert the y coordinate so higher of y go up.
        let y = height - y as u32;

        let row_iter = row
            .as_chunks_mut().0 // &mut [[u8; 3]]
            .iter_mut()        // impl Iterator<Item = &mut [u8; 3]>
            .map(rgb_mut_ref); // impl Iterator<Item = &mut Rgb<u8>>

        for (x, pixel) in row_iter.enumerate() {
            let mut color = Color::black();
            for _ in 0..samples_per_pixel {
                let u = (x as f64 + rng.gen::<f64>()) / (width  as f64 - 1.0);
                let v = (y as f64 + rng.gen::<f64>()) / (height as f64 - 1.0);

                color += camera.get_ray(u, v).compute_color(&world, max_bounces);
            }
            *pixel = (color / (samples_per_pixel as f64)).sqrt().into();

            let oldval = count.fetch_add(1, Ordering::SeqCst);
            if oldval % 60 == 0 {
                let percent = (oldval as f64 * 100.0) / (width * height) as f64;
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

pub fn single_thread_render(scene: Scene) {
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
        let mut color = Color::black();

        for _ in 0..samples_per_pixel {
            let mut rng = thread_rng();
            let u = (x as f64 + rng.gen_range(0.0..1.0)) / (width  as f64 - 1.0);
            let v = ((height - y) as f64 + rng.gen_range(0.0..1.0)) / (height as f64 - 1.0);
            color += camera.get_ray(u, v).compute_color(&world, max_bounces)
        }

        *pixel = (color / (samples_per_pixel as f64)).sqrt().into();

        count += 1;
        let percent = (count as f64 * 100.0) / (width * height) as f64;
        eprint!("\r[{:03.0}%] Rendering", percent);
    }

    eprintln!("\nDone!");
}
