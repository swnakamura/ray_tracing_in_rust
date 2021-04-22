use super::*;
pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    lens_radius: f64,
    u: Vec3,
    v: Vec3,
    w: Vec3,
}

impl Camera {
    /// Initializes `Camera`.
    /// * `vfov` - Vertical fov in degrees.
    /// * `aperture` - Effective lens diameter
    pub fn new(
        lookfrom: Point3,
        lookat: Point3,
        vup: Vec3,
        vfov: f64,
        aspect_ratio: f64,
        aperture: f64,
        focus_dist: f64,
    ) -> Self {
        let theta = vfov.to_radians();
        let h = (theta / 2.).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        // define camera coordinates
        let w = (lookfrom - lookat).normalize();
        let u = vup.cross(&w).normalize();
        let v = w.cross(&u);

        let origin = lookfrom;
        let horizontal = u * viewport_width;
        let vertical = v * viewport_height;
        let lower_left_corner = origin - horizontal / 2. - vertical / 2. - w * focus_dist;
        let lens_radius = aperture / 2.;

        Self {
            origin,
            horizontal,
            vertical,
            lower_left_corner,
            lens_radius,
            u,
            v,
            w,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd = Point3::random_in_unit_disk() * self.lens_radius;
        let offset = self.u * rd.x() + self.v * rd.y();
        let light_origin = self.origin + offset;
        Ray::new(
            light_origin,
            self.lower_left_corner + self.horizontal * s + self.vertical * t - light_origin,
        )
    }
}
