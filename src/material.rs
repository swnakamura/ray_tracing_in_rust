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
        let mut scatter_direction = rec.normal + random_unit_vector();

        // Catch degenerate scatter direction
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        let scattered = Ray::new(rec.p, scatter_direction);
        return Some((scattered, self.albedo));
    }
}

pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        let reflect_direction = r_in.direction().reflect(&rec.normal);

        let scattered = Ray::new(rec.p, reflect_direction + random_unit_vector() * self.fuzz);
        // TODO:Isn't this always positive?
        if rec.front_face {
            Some((scattered, self.albedo))
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

pub struct Dielectric {
    /// index of refraction
    ir: f64,
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        let refraction_ratio = match rec.front_face {
            false => self.ir,
            true => 1. / self.ir,
        };

        let normalized_r = r_in.direction().normalize();
        let n = &rec.normal;
        let cos_theta = -normalized_r.dot(&n);
        let sin_theta = (1. - cos_theta * cos_theta).sqrt();
        let cannot_refract = refraction_ratio * sin_theta > 1.;
        let direction = if cannot_refract
            || Dielectric::reflectance(cos_theta, refraction_ratio) > random::<f64>()
        {
            // the ray didn't refract, but reflected
            r_in.direction().reflect(&n)
        } else {
            r_in.direction().refract(&rec.normal, refraction_ratio)
        };
        let refracted_ray = Ray::new(rec.p, direction);

        Some((refracted_ray, Color::new(1., 1., 1.)))
    }
}

impl Dielectric {
    pub fn new(ir: f64) -> Self {
        Self { ir }
    }
    fn reflectance(cosine: f64, ir: f64) -> f64 {
        // Use Schlick's approximation for reflectance.
        let mut r0 = (1. - ir) / (1. + ir);
        r0 = r0 * r0;
        return r0 + (1. - r0) * (1. - cosine).powi(5);
    }
}
