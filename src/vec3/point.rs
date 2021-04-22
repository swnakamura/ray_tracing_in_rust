pub type Point3 = super::Vec3;

impl Point3 {
    pub fn random_in_unit_sphere() -> Point3 {
        let mut p;
        while {
            p = Point3::random_in_range(-1., 1.);
            p.length_squared() >= 1.
        } {}
        return p;
    }
    pub fn random_unit_vector() -> Point3 {
        Point3::random_in_unit_sphere().normalize()
    }

    pub fn random_in_unit_disk() -> Point3 {
        use rand::prelude::random;
        fn linearmap(val: f64, min: f64, max: f64) -> f64 {
            val * (max - min) + min
        }

        let mut p;
        while {
            p = Point3::new(
                linearmap(random(), -1., 1.),
                linearmap(random(), -1., 1.),
                0.,
            );
            p.length_squared() >= 1.
        } {}
        return p;
    }
}
