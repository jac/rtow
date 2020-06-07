use std::io::{self, Write};
mod tracing;
mod vec3;

use tracing::Ray;
use vec3::{Colour, Point3, Vec3};

fn hit_sphere(center: &Point3, radius: f64, ray: &Ray) -> f64 {
    // Quadratic formula
    let oc = ray.origin - *center;
    let a = ray.direction.length_squared();
    let half_b = Vec3::dot(oc, ray.direction);
    let c = oc.length_squared() - radius * radius;
    let discriminant = half_b * half_b - a * c;
    if discriminant < 0.0 {
        -1.0
    } else {
        (-half_b - discriminant.sqrt()) / (2.0 * a)
    }
}

fn ray_colour(ray: &Ray) -> Colour {
    let t = hit_sphere(&Point3::from((0.0, 0.0, -1.0)), 0.5, &ray);
    if t > 0.0 {
        let N = Vec3::unit_vector(ray.at(t) - Vec3::from((0.0, 0.0, -1.0)));
        0.5 * Colour::from((N.x + 1.0, N.y + 1.0, N.z + 1.0))
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

    // PPM header
    let header = format!("P3\n{} {}\n255\n", image_width, image_height);
    handle
        .write_all(header.as_bytes())
        .expect("Failed to write header");

    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::new();
    let horizontal = Vec3::from((viewport_width, 0.0, 0.0));
    let vertical = Vec3::from((0.0, viewport_height, 0.0));
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3::from((0.0, 0.0, focal_length));

    // Image content
    for j in (0..=image_height - 1).rev() {
        for i in 0..image_width {
            let u = (i as f64) / ((image_width - 1) as f64);
            let v = (j as f64) / ((image_height - 1) as f64);

            let ray = Ray::new(
                origin,
                lower_left_corner + u * horizontal + v * vertical - origin,
            );

            let pixel_colour = ray_colour(&ray);
            vec3::write_colour(&mut handle, pixel_colour);
        }
    }
}
