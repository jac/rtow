use super::vec3::Colour;
use crate::util::clamp;
use std::fmt::{self, Write};

impl Colour {
    pub fn write_colour(
        &self,
        output: &mut String,
        samples_per_pixel: usize,
    ) -> Result<(), fmt::Error> {
        let mut r = self.x;
        let mut g = self.y;
        let mut b = self.z;

        let scale = 1.0 / samples_per_pixel as f64;
        r = (scale * r).sqrt();
        g = (scale * g).sqrt();
        b = (scale * b).sqrt();

        writeln!(
            output,
            "{} {} {}",
            (256.0 * clamp(r, 0.0, 0.999)) as u8,
            (256.0 * clamp(g, 0.0, 0.999)) as u8,
            (256.0 * clamp(b, 0.0, 0.999)) as u8
        )
    }
}
