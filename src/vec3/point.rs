pub type Point3 = super::Vec3;

pub fn random_in_unit_sphere() -> Point3 {
    let mut p;
    while {
        p = Point3::random_in_range(-1., 1.);
        p.length_squared() >= 1.
    } {}
    return p;
}
