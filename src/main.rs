mod tracing;

use std::io::{self, Write as ioWrite};
use std::fmt::Write as fmtWrite;
use tracing::{Colour, Ray, Point3, Vec3};
use std::error::Error;

fn ray_colour(ray: &Ray) -> Colour {
    let unit_direction = ray.direction.unit_vector();
    let t = 0.5 * (unit_direction.y + 1.0);
    (1.0 - t) * Colour::new(1.0, 1.0, 1.0) + t * Colour::new(0.5, 0.7, 1.0)
}

fn main() -> Result<(), Box<dyn Error>> {
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: usize = 384;
    const IMAGE_HEIGHT: usize = ((IMAGE_WIDTH as f64) / ASPECT_RATIO) as usize;

    // Stdout for PPM data
    let stdout = io::stdout();
    let stdout = &mut stdout.lock();

    // Stderr for progress data
    let stderr = io::stderr();
    let stderr = &mut stderr.lock();
    let text_width = ((IMAGE_HEIGHT as f64).log10() as usize) + 1;

    // Parameters
    const VIEWPORT_HEIGHT: f64 = 2.0;
    const VIEWPORT_WIDTH: f64 = ASPECT_RATIO * VIEWPORT_HEIGHT;
    const FOCAL_LENGTH: f64 = 1.0;

    const ORIGIN: Point3 = Point3::new(0.0, 0.0, 0.0);
    const HORIZONTAL: Vec3 = Vec3::new(VIEWPORT_WIDTH, 0.0, 0.0);
    const VERTICAL: Vec3 = Vec3::new(0.0, VIEWPORT_HEIGHT, 0.0);
    let lower_left_corner: Vec3 = ORIGIN - HORIZONTAL / 2.0 - VERTICAL / 2.0 - Vec3::new(0.0, 0.0, FOCAL_LENGTH);

    let mut output = String::with_capacity(1_000_000);
    // PPM Header
    writeln!(&mut output, "P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT)?;
    // Image Contents
    for vert in (0..IMAGE_HEIGHT).rev() {
        write!(
            stderr,
            "\rScanlines remaining: {0:1$}",
            vert + 1,
            text_width
        )?;

        for hor in 0..IMAGE_WIDTH {
            let u = (hor as f64) / ((IMAGE_WIDTH as f64) - 1.0);
            let v = (vert as f64) / ((IMAGE_HEIGHT as f64) - 1.0);
            let ray = Ray::new(ORIGIN, lower_left_corner + u * HORIZONTAL + v * VERTICAL - ORIGIN);
            let pixel_colour = ray_colour(&ray);
            pixel_colour.write_colour(&mut output);
        }
    }
    stdout.write_all(output.as_bytes())?;
    writeln!(stderr, "\nDone!")?;
    Ok(())
}
