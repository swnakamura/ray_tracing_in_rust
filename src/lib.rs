pub mod camera;
pub mod colors;
pub mod hittable;
pub mod ray;
pub mod sphere;
pub mod vec3;

use camera::Camera;
use colors::write_color;
use hittable::HittableList;
use rand::prelude::*;
use ray::Ray;
use sphere::Sphere;
use std::rc::Rc;
use vec3::{
    color::Color,
    point::{random_in_unit_sphere, Point3},
    Vec3,
};

pub fn ray_color(r: Ray, world: &HittableList, depth: i64) -> Color {
    // if too deep, no more light is gathered
    if depth <= 0 {
        return Color::new([0., 0., 0.]);
    }
    // if hit, return that color
    if let Some(rec) = world.hit(&r, 0.001, std::f64::INFINITY) {
        return ray_color(
            Ray::new(rec.p.clone(), rec.normal + random_in_unit_sphere()),
            &world,
            depth - 1,
        ) * 0.5;
    }
    // else, background light
    let unit_direction = r.direction().normalize();
    let t = (unit_direction.y() + 1.0) * 0.5;
    Color::new([1.0, 1.0, 1.0]) * (1.0 - t) + Color::new([0.5, 0.7, 1.0]) * t
}

pub fn render() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as i32;
    let samples_per_pixel = 100;
    let max_depth = 50;

    // World
    let mut world = HittableList::new();
    world.add(Rc::new(Sphere::new(Point3::new([0., 0., -1.]), 0.5)));
    world.add(Rc::new(Sphere::new(Point3::new([0., -100.5, -1.]), 100.)));

    // Camera
    let cam = Camera::new();

    // rand generator for antialiasing
    let mut rng = thread_rng();

    // Render
    print!("P3\n{} {}\n255\n", image_width, image_height);

    for h in (0..image_height).rev() {
        eprint!("\rScanlines remaining: {} ", h);
        for w in 0..image_width {
            let mut pixel_color = Color::new([0., 0., 0.]);
            for _s in 0..samples_per_pixel {
                let (u, v) = (
                    (w as f64 + rng.gen::<f64>()) / (image_width - 1) as f64,
                    (h as f64 + rng.gen::<f64>()) / (image_height - 1) as f64,
                );
                let r = cam.get_ray(u, v);
                pixel_color += ray_color(r, &world, max_depth);
            }
            write_color(pixel_color, samples_per_pixel);
        }
    }
    eprintln!("\nDone.");
}
