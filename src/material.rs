use super::*;
use hittable::HitRecord;

pub trait Material {
    /// Returns reflecting ray and its color attenuation
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)>;
}

pub struct Lambertian {
    pub albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        let mut scatter_direction = rec.normal.clone() + random_unit_vector();

        // Catch degenerate scatter direction
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal.clone();
        }

        let scattered = Ray::new(rec.p.clone(), scatter_direction);
        return Some((scattered, self.albedo.clone()));
    }
}

pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        let reflect_direction = r_in.direction().reflect(&rec.normal);

        let scattered = Ray::new(
            rec.p.clone(),
            reflect_direction + random_unit_vector() * self.fuzz,
        );
        // TODO:Isn't this always positive?
        if rec.front_face {
            Some((scattered, self.albedo.clone()))
        } else {
            None
        }
    }
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        Self {
            albedo,
            fuzz: if fuzz < 1. { fuzz } else { 1. },
        }
    }
}
