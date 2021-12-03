#![feature(slice_as_chunks)]
#![feature(atomic_from_mut)]
#![allow(dead_code)]
#![allow(unused_imports)]

use std::default::Default;

mod utils;
mod ray;
mod objects;
mod material;
mod hittable;
mod camera;
mod render;

use objects::{ Sphere, WorldBuilder };
use camera::Camera;
use render::{
    multi_thread_render,
    simple_multi_thread_render,
    single_thread_render,
    RenderBuilder,
    Scene,
};

fn main() {
    let aspect_ratio = 3.0 / 2.0;

    let render = RenderBuilder::new()
        .with_ratio(aspect_ratio, 720)
        .with_samples(100)
        .with_max_bounces(10)
        .build();

    let look_from  = nalgebra_glm::vec3(13.0, 2.0, 3.0);
    let look_at    = nalgebra_glm::vec3(0.0, 0.0, 0.0);
    let vup        = nalgebra_glm::vec3(0.0, 1.0, 0.0);
    let focus_dist = 10.0;
    let aperture   = 0.1;

    let camera = Camera::new(
        look_from,
        look_at,
        vup,
        20.0,
        aspect_ratio,
        aperture,
        focus_dist,
    );

    /*
    let material_ground = material::Diffuse::new(Color::new(0.8, 0.8, 0.0));
    let material_center = material::Diffuse::new(Color::new(0.1, 0.2, 0.5));
    let material_left   = material::Dielectric::new(1.5);
    let material_right  = material::Metal::new(Color::new(0.8, 0.6, 0.2), 0.0);

    let world = WorldBuilder::default()
        .add(Sphere::new(Point3::new( 0.0, -100.5, -1.0), 100.0, material_ground.clone()))
        .add(Sphere::new(Point3::new( 0.0,    0.0, -1.0),   0.5, material_center.clone()))
        .add(Sphere::new(Point3::new(-1.0,    0.0, -1.0),   0.5, material_left.clone()))
        .add(Sphere::new(Point3::new(-1.0,    0.0, -1.0), -0.45, material_left.clone()))
        .add(Sphere::new(Point3::new( 1.0,    0.0, -1.0),   0.5, material_right.clone()))
        .build();
    */

    let world = render::random_scene();

    multi_thread_render(Scene::new(world, camera, render));
}
