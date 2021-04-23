pub mod camera;
pub mod colors;
pub mod hittable;
pub mod material;
pub mod ray;
pub mod sphere;
pub mod vec3;

use camera::Camera;
use colors::write_color;
use hittable::HittableList;
#[allow(unused_imports)]
use material::{Dielectric, Lambertian, Metal};
use rand::prelude::*;
use ray::Ray;
use sphere::Sphere;
use std::sync::*;
use vec3::{color::Color, point::Point3, Vec3};

pub fn ray_color(r: Ray, world: &HittableList, depth: isize) -> Color {
    // if we've exceeded the ray bounce limit, no more light is computed
    if depth <= 0 {
        return Color::new(0., 0., 0.);
    }
    // if hit, return that color
    if let Some(rec) = world.hit(&r, 0.001, std::f64::INFINITY) {
        if let Some((scattered_ray, attenuation)) = rec.mat_ptr.clone().unwrap().scatter(&r, &rec) {
            return ray_color(scattered_ray, &world, depth - 1) * attenuation;
        }
        return Color::new(0., 0., 0.);
    }
    // else, background light
    let unit_direction = r.direction().normalize();
    let t = (unit_direction.y() + 1.0) * 0.5;
    Color::new(1.0, 1.0, 1.0) * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t
}

pub fn render() {
    // Image
    let aspect_ratio = 3. / 2.;
    let image_width = 1200;
    let image_height = (image_width as f64 / aspect_ratio) as i32;
    let samples_per_pixel = 500;
    let max_depth = 50;

    // World
    let world = random_scene();

    // Camera
    let lookfrom = Point3::new(13., 2., 3.);
    let lookat = Point3::new(0., 0., 0.);
    let vup = Vec3::new(0., 1., 0.);
    let dist_to_focus = 10.;
    let aperture = 0.1;
    let cam = Camera::new(
        lookfrom,
        lookat,
        vup,
        120.,
        aspect_ratio,
        aperture,
        dist_to_focus,
    );

    // Render
    print!("P3\n{} {}\n255\n", image_width, image_height);

    for h in (0..image_height).rev() {
        eprint!("\rScanlines remaining: {} ", h);
        for w in 0..image_width {
            use rayon::prelude::*;
            let pixel_color = (0..samples_per_pixel)
                .into_par_iter()
                .map(|_| {
                    let (u, v) = (
                        (w as f64 + random::<f64>()) / (image_width - 1) as f64,
                        (h as f64 + random::<f64>()) / (image_height - 1) as f64,
                    );
                    let r = cam.get_ray(u, v);
                    ray_color(r, &world, max_depth)
                })
                .sum();
            write_color(pixel_color, samples_per_pixel);
        }
    }
    eprintln!("\nDone.");
}

fn random_scene() -> HittableList {
    let mut world = HittableList::new();

    let ground_material = Arc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    world.add(Arc::new(Sphere::new(
        Point3::new(0., -1000., 0.),
        1000.,
        ground_material,
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat: f64 = random();
            let center = Point3::new(
                a as f64 + 0.9 * random::<f64>(),
                0.2,
                b as f64 + 0.9 * random::<f64>(),
            );

            if (center - Point3::new(4., 0.2, 0.)).length() > 0.9 {
                if choose_mat < 0.7 {
                    // diffuse
                    let albedo = Color::random() * Color::random();
                    let sphere_material = Arc::new(Lambertian::new(albedo));
                    world.add(Arc::new(Sphere::new(center, 0.2, sphere_material)));
                } else if choose_mat < 0.8 {
                    // metal
                    let albedo = Color::random_in_range(0.5, 1.);
                    let fuzz = random::<f64>() * 0.5;
                    let sphere_material = Arc::new(Metal::new(albedo, fuzz));
                    world.add(Arc::new(Sphere::new(center, 0.2, sphere_material)));
                } else {
                    // glass
                    let sphere_material = Arc::new(Dielectric::new(1.5));
                    world.add(Arc::new(Sphere::new(center, 0.2, sphere_material)));
                }
            }
        }
    }

    let material1 = Arc::new(Dielectric::new(1.5));
    world.add(Arc::new(Sphere::new(
        Point3::new(0., 1., 0.),
        1.,
        material1,
    )));

    let material2 = Arc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    world.add(Arc::new(Sphere::new(
        Point3::new(-4., 1., 0.),
        1.,
        material2,
    )));

    let material3 = Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Arc::new(Sphere::new(
        Point3::new(4., 1., 0.),
        1.,
        material3,
    )));

    world
}
