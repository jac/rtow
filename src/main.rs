mod tracing;
mod util;

use std::error::Error;
use std::f64::INFINITY;
use std::fmt::Write as fmtWrite;
use std::io::{self, Write as ioWrite};
use std::rc::Rc;
use tracing::{Camera, Colour, Hittable, HittableList, Lambertian, Metal, Point3, Ray, Sphere};
use util::random;

fn ray_colour(ray: &Ray, world: &HittableList, depth: usize) -> Colour {
    if depth == 0 {
        Colour::default()
    } else if let Some(hit) = world.hit(ray, 0.001, INFINITY) {
        if let Some((colour, scattered)) = hit.material.scatter(ray, &hit) {
            colour * ray_colour(&scattered, world, depth - 1)
        } else {
            Colour::default()
        }
    } else {
        let unit_direction = ray.direction.unit_vector();
        let t = 0.5 * (unit_direction.y + 1.0);
        (1.0 - t) * Colour::new(1.0, 1.0, 1.0) + t * Colour::new(0.5, 0.7, 1.0)
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: usize = 384;
    const IMAGE_HEIGHT: usize = ((IMAGE_WIDTH as f64) / ASPECT_RATIO) as usize;
    const SAMPLES_PER_PIXEL: usize = 100;
    const MAX_DEPTH: usize = 50;

    // Stdout for PPM data
    let stdout = io::stdout();
    let stdout = &mut stdout.lock();

    // Stderr for progress data
    let stderr = io::stderr();
    let stderr = &mut stderr.lock();
    let text_width = ((IMAGE_HEIGHT as f64).log10() as usize) + 1;

    let mut output = String::with_capacity(1_000_000);
    // PPM Header
    writeln!(&mut output, "P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT)?;
    // Image Contents
    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, 0.0, -1.0),
        0.5,
        Rc::new(Lambertian::new(Colour::new(0.7, 0.3, 0.3))),
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        Rc::new(Lambertian::new(Colour::new(0.8, 0.8, 0.0))),
    )));

    world.add(Box::new(Sphere::new(
        Point3::new(1.0, 0.0, -1.0),
        0.5,
        Rc::new(Metal::new(Colour::new(0.8, 0.6, 0.2), 0.3)),
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        0.5,
        Rc::new(Metal::new(Colour::new(0.8, 0.8, 0.8), 1.0)),
    )));

    let cam = Camera::new();

    for vert in (0..IMAGE_HEIGHT).rev() {
        write!(
            stderr,
            "\rScanlines remaining: {0:1$}",
            vert + 1,
            text_width
        )?;

        for hor in 0..IMAGE_WIDTH {
            let mut pixel_colour = Colour::default();
            for _ in 0..SAMPLES_PER_PIXEL {
                let u = (random() + hor as f64) / ((IMAGE_WIDTH as f64) - 1.0);
                let v = (random() + vert as f64) / ((IMAGE_HEIGHT as f64) - 1.0);

                let ray = cam.get_ray(u, v);
                pixel_colour += ray_colour(&ray, &world, MAX_DEPTH);
            }
            pixel_colour.write_colour(&mut output, SAMPLES_PER_PIXEL)?;
        }
    }
    stdout.write_all(output.as_bytes())?;
    writeln!(stderr, "\nDone!")?;
    Ok(())
}
