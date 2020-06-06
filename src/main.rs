use std::io::{self, Write};
mod colour;
mod vec3;

use vec3::Colour;

fn main() {
    // Acquire a handle to stdout
    let stdout = io::stdout();
    let mut handle = stdout.lock();

    let image_width = 256;
    let image_height = 256;

    // PPM header
    let header = format!("P3\n{} {}\n255\n", image_width, image_height);
    handle.write_all(header.as_bytes()).expect("Failed to write header");

    // Image content
    for j in (0..=image_height-1).rev() {
        for i in 0..image_width {
            let r = (i as f64) / ((image_width-1) as f64);
            let g = (j as f64) / ((image_height-1) as f64);
            let b = 0.25;

            let pixel = Colour::from((r, g, b));
            colour::write_colour(&mut handle, pixel);
        }
    }
    
}
