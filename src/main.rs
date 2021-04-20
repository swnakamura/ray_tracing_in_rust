use ray_tracing_in_rust::ray::Ray;
use ray_tracing_in_rust::vec3::{color::Color, point::Point3, Vec3};

fn ray_color(r: Ray) -> Color {
    let unit_direction = r.direction().normalize();
    let t = (unit_direction.y() + 1.0) * 0.5;
    Color::new([1.0, 1.0, 1.0]) * (1.0 - t) + Color::new([0.5, 0.7, 1.0]) * t
}

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as i32;

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
            let pixel_color = ray_color(r);
            pixel_color.write_color();
        }
    }
    eprintln!("\nDone.");
}
