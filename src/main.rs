mod tracing;

use std::io::{self, Write};
use tracing::Colour;

const IMAGE_WIDTH: usize = 256;
const IMAGE_HEIGHT: usize = 256;

fn main() -> Result<(), io::Error> {
    // Stdout for PPM data
    let stdout = io::stdout();
    let stdout = &mut stdout.lock();

    // Stderr for progress data
    let stderr = io::stderr();
    let stderr = &mut stderr.lock();
    let text_width = ((IMAGE_HEIGHT as f64).log10() as usize) + 1;

    // PPM Header
    writeln!(stdout, "P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT)?;

    // Image Contents
    for vert in (0..IMAGE_HEIGHT).rev() {
        write!(
            stderr,
            "\rScanlines remaining: {0:1$}",
            vert + 1,
            text_width
        )?;

        for hor in 0..IMAGE_WIDTH {
            let pixel_colour = Colour::new(
                (hor as f64) / ((IMAGE_WIDTH as f64) - 1.0),
                (vert as f64) / ((IMAGE_HEIGHT as f64) - 1.0),
                0.25,
            );
            pixel_colour.write_colour(stdout)?;
        }
    }

    writeln!(stderr, "\nDone!")?;
    Ok(())
}
