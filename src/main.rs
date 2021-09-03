#![feature(slice_as_chunks)]
#![allow(dead_code)]
#![allow(unused_imports)]

use std::default::Default;

mod vec3;
mod ray;
mod objects;
mod material;
mod hittable;
mod camera;
mod render;

use vec3::{ Point3, Color };
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
    let aspect_ratio = 16.0 / 9.0;

    let render = RenderBuilder::new()
        .with_ratio(aspect_ratio, 1080)
        .with_samples(300)
        .with_max_bounces(10)
        .build();

    let camera = Camera::new(aspect_ratio);

    let material_ground = material::Diffuse::new(Color::new(0.8, 0.8, 0.0));
    let material_center = material::Diffuse::new(Color::new(0.7, 0.3, 0.3));
    let material_left   = material::Metal::new(Color::new(0.8, 0.8, 0.8), 0.3);
    let material_right  = material::Metal::new(Color::new(0.8, 0.6, 0.2), 0.1);

    let world = WorldBuilder::default()
        .add(Sphere::new(Point3::new( 0.0, -100.5, -1.0), 100.0, material_ground))
        .add(Sphere::new(Point3::new( 0.0,    0.0, -1.0),   0.5, material_center))
        .add(Sphere::new(Point3::new(-1.0,    0.0, -1.0),   0.5, material_left))
        .add(Sphere::new(Point3::new( 1.0,    0.0, -1.0),   0.5, material_right))
        .build();

    simple_multi_thread_render(Scene::new(world, camera, render));
}
