use super::vec3::Colour;
use std::io::{self, StdoutLock, Write};

impl Colour {
    pub fn write_colour(&self, stdout: &mut StdoutLock) -> Result<(), io::Error> {
        writeln!(
            stdout,
            "{} {} {}",
            (255.99 * self.x) as u8,
            (255.99 * self.y) as u8,
            (255.99 * self.z) as u8
        )?;
        Ok(())
    }
}
