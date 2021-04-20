mod vec3;
use vec3::Vec3;

fn main() {
    // Image
    let image_width = 256;
    let image_height = 256;

    print!("P3\n{} {}\n255\n", image_height, image_height);

    // Render
    for h in 0..image_height {
        eprint!("\rScanlines remaining: {} ", h);
        for w in 0..image_width {
            let v = Vec3::new([
                h as f64 / (image_height - 1) as f64,
                w as f64 / (image_width - 1) as f64,
                0.25,
            ]);
            v.write_color();
        }
    }
    eprintln!("\nDone.");
}
