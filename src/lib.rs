pub mod ray;
pub mod vec3;
use ray::Ray;
use vec3::{color::Color, point::Point3};

fn hit_sphere(center: Point3, radius: f64, r: Ray) -> f64 {
    let oc = r.origin() - center;
    let a = r.direction().length_squared();
    let b = 2.0 * r.direction().dot(&oc);
    let c = oc.length_squared() - radius * radius;
    let discriminant = b * b - 4.0 * a * c;
    if discriminant < 0. {
        -1.0
    } else {
        (-b - discriminant.sqrt()) / (2.0 * a)
    }
}

pub fn ray_color(r: Ray) -> Color {
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
