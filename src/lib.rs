pub mod hittable;
pub mod ray;
pub mod sphere;
pub mod vec3;

use hittable::HittableList;
use ray::Ray;
use sphere::Sphere;
use std::rc::Rc;
use vec3::{color::Color, point::Point3, Vec3};

fn hit_sphere(center: Point3, radius: f64, r: Ray) -> f64 {
    let oc = r.origin() - center;
    let a = r.direction().length_squared();
    let half_b = r.direction().dot(&oc);
    let c = oc.length_squared() - radius * radius;
    let discriminant = half_b * half_b - a * c;
    if discriminant < 0. {
        -1.0
    } else {
        (-half_b - discriminant.sqrt()) / a
    }
}

pub fn ray_color(r: Ray, world: &HittableList) -> Color {
    if let Some(rec) = world.hit(&r, 0., std::f64::INFINITY) {
        return (rec.normal + Color::new([1., 1., 1.])) * 0.5;
    }
    let center_of_the_sphere = Point3::new([0., 0., -1.]);
    let t = hit_sphere(center_of_the_sphere.clone(), 0.5, r.clone());
    if t > 0.0 {
        let n = (r.at(t) - center_of_the_sphere).normalize();
        return Color::new([n.x() + 1.0, n.y() + 1.0, n.z() + 1.0]) * 0.5;
    }
    let unit_direction = r.direction().normalize();
    let t = (unit_direction.y() + 1.0) * 0.5;
    Color::new([1.0, 1.0, 1.0]) * (1.0 - t) + Color::new([0.5, 0.7, 1.0]) * t
}

pub fn render() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as i32;

    // World
    let mut world = HittableList::new();
    world.add(Rc::new(Sphere::new(Point3::new([0., 0., -1.]), 0.5)));
    world.add(Rc::new(Sphere::new(Point3::new([0., -100.5, -1.]), 100.)));

    // Camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::new([0., 0., 0.]);
    let horizontal = Vec3::new([viewport_width, 0., 0.]);
    let vertical = Vec3::new([0., viewport_height, 0.]);
    let lower_left_corner = origin.clone()
        - horizontal.clone() / 2.
        - vertical.clone() / 2.
        - Vec3::new([0., 0., focal_length]);

    // Render
    //
    print!("P3\n{} {}\n255\n", image_width, image_height);

    for h in (0..image_height).rev() {
        eprint!("\rScanlines remaining: {} ", h);
        for w in 0..image_width {
            let (u, v) = (
                w as f64 / (image_width - 1) as f64,
                h as f64 / (image_height - 1) as f64,
            );
            let r = Ray::new(
                origin.clone(),
                lower_left_corner.clone() + horizontal.clone() * u + vertical.clone() * v
                    - origin.clone(),
            );
            let pixel_color = ray_color(r, &world);
            pixel_color.write_color();
        }
    }
    eprintln!("\nDone.");
}
