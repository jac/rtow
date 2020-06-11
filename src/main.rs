use std::io::{self, Write};

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
        write!(stderr, "\rScanlines remaining: {0:1$}", vert + 1, text_width)?;
        stderr.flush()?;

        for hor in 0..IMAGE_WIDTH {
            let r = (hor as f64) / (IMAGE_WIDTH - 1) as f64;
            let g = (vert as f64) / (IMAGE_HEIGHT - 1) as f64;
            let b = 0.25;

            let ir = (255.99 * r) as u8;
            let ig = (255.99 * g) as u8;
            let ib = (255.99 * b) as u8;

            writeln!(stdout, "{} {} {}", ir, ig, ib)?;
        }
    }

    writeln!(stderr, "\nDone!")?;
    Ok(())
}
