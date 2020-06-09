use std::f64::INFINITY;
use std::io::{self, Write};

mod tracing;
mod util;
mod vec3;

use tracing::{Camera, HitRecord, Hittable, HittableList, Ray, Sphere};
use vec3::{Colour, Point3, Vec3};

fn ray_colour(ray: &Ray, world: &dyn Hittable, depth: usize) -> Colour {
    if depth <= 0 {
        return Colour::new();
    }

    let mut rec = HitRecord::init_zero();
    if world.hit(ray, 0.001, INFINITY, &mut rec) {
        // Random scattering
        // let target = rec.p + rec.normal + Vec3::random_in_unit_sphere();
        // True lambertian
        // let target = rec.p + rec.normal + Vec3::random_unit_vector();
        // Hemispherical scattering
        let target = rec.p + Vec3::random_in_hemisphere(&rec.normal);
        0.5 * ray_colour(&Ray::new(rec.p, target - rec.p), world, depth - 1)
    } else {
        let unit_direction = Vec3::unit_vector(ray.direction);
        let t = 0.5 * (unit_direction.y + 1.0);
        (1.0 - t) * Colour::from((1.0, 1.0, 1.0)) + t * Colour::from((0.5, 0.7, 1.0))
    }
}

fn main() {
    // Acquire a handle to stdout
    let stdout = io::stdout();
    let mut handle = stdout.lock();

    let aspect_ratio = 16.0 / 9.0;
    let image_width = 384;
    let image_height = (image_width as f64 / aspect_ratio) as u32;
    let samples_per_pixel = 100;
    let max_depth = 50;

    // PPM header
    let header = format!("P3\n{} {}\n255\n", image_width, image_height);
    handle
        .write_all(header.as_bytes())
        .expect("Failed to write header");

    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(Point3::from((0.0, 0.0, -1.0)), 0.5)));
    world.add(Box::new(Sphere::new(
        Point3::from((0.0, -100.5, -1.0)),
        100.0,
    )));

    let cam = Camera::new();

    // Image content
    for j in (0..=image_height - 1).rev() {
        for i in 0..image_width {
            let mut pixel_colour = Colour::new();
            for _ in 0..samples_per_pixel {
                let u = (i as f64 + util::random()) / ((image_width - 1) as f64);
                let v = (j as f64 + util::random()) / ((image_height - 1) as f64);
                let ray = cam.get_ray(u, v);
                pixel_colour += ray_colour(&ray, &world, max_depth);
            }
            vec3::write_colour(&mut handle, pixel_colour, samples_per_pixel);
        }
    }
}
