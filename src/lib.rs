pub mod ray;
pub mod vec3;
use ray::Ray;
use vec3::{color::Color, point::Point3};

fn hit_sphere(center: Point3, radius: f64, r: Ray) -> bool {
    let oc = r.origin() - center;
    let a = r.direction().length_squared();
    let b = 2.0 * r.direction().dot(&oc);
    let c = oc.length_squared() - radius * radius;
    let discriminant = b * b - 4.0 * a * c;
    discriminant > 0.
}

pub fn ray_color(r: Ray) -> Color {
    if hit_sphere(Point3::new([0., 0., -1.]), 0.5, r.clone()) {
        return Color::new([1., 0., 0.]);
    }
    let unit_direction = r.direction().normalize();
    let t = (unit_direction.y() + 1.0) * 0.5;
    Color::new([1.0, 1.0, 1.0]) * (1.0 - t) + Color::new([0.5, 0.7, 1.0]) * t
}
