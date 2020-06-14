use super::vec3::Colour;
use std::fmt::Write;

impl Colour {
    pub fn write_colour(&self, output: &mut String) {
        writeln!(
            output,
            "{} {} {}",
            (255.99 * self.x) as u8,
            (255.99 * self.y) as u8,
            (255.99 * self.z) as u8
        )
        .expect("OKay");
    }
}
