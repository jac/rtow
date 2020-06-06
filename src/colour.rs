use std::io::{self, Write};
use super::vec3::Colour;

pub fn write_colour(stdout: &mut io::StdoutLock, colour: Colour) {
    let pixel = 
        format!("{} {} {}\n", 
        (255.99 * colour.x) as u8, 
        (255.99 * colour.y) as u8, 
        (255.99 * colour.z) as u8);

    stdout.write_all(pixel.as_bytes()).expect("Could not write colour");
}