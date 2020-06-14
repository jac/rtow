use super::vec3::Colour;
use crate::util::clamp;
use std::fmt::{self, Write};

impl Colour {
    pub fn write_colour(
        &mut self,
        output: &mut String,
        samples_per_pixel: usize,
    ) -> Result<(), fmt::Error> {
        let scale = 1.0 / samples_per_pixel as f64;
        *self *= scale;

        writeln!(
            output,
            "{} {} {}",
            (256.0 * clamp(self.x, 0.0, 0.999)) as u8,
            (256.0 * clamp(self.y, 0.0, 0.999)) as u8,
            (256.0 * clamp(self.z, 0.0, 0.999)) as u8
        )
    }
}
