use super::*;
pub fn write_color(mut colors: Color, samples_per_pixel: isize) {
    let scale = 1.0 / samples_per_pixel as f64;
    colors *= scale;
    let [mut r, mut g, mut b] = colors.e;

    // gamma-correct for gamma=2.0
    r = r.sqrt();
    g = g.sqrt();
    b = b.sqrt();

    fn clamp(x: f64, min: f64, max: f64) -> f64 {
        if x < min {
            min
        } else if x > max {
            max
        } else {
            x
        }
    }

    println!(
        "{} {} {}",
        (256. * clamp(r, 0.0, 0.999)) as i32,
        (256. * clamp(g, 0.0, 0.999)) as i32,
        (256. * clamp(b, 0.0, 0.999)) as i32,
    );
}
