use super::Colour;
use crate::util;
use std::io::{self, Write};

pub fn write_colour(stdout: &mut io::StdoutLock, pixel_colour: Colour, samples_per_pixel: usize) {
    let mut r = pixel_colour.x;
    let mut g = pixel_colour.y;
    let mut b = pixel_colour.z;

    let scale = 1.0 / samples_per_pixel as f64;
    r = (scale * r).sqrt();
    g = (scale * g).sqrt();
    b = (scale * b).sqrt();

    let pixel = format!(
        "{} {} {}\n",
        (256.0 * util::clamp(r, 0.0, 0.999)) as u8,
        (256.0 * util::clamp(g, 0.0, 0.999)) as u8,
        (256.0 * util::clamp(b, 0.0, 0.999)) as u8
    );

    stdout
        .write_all(pixel.as_bytes())
        .expect("Could not write colour");
}
