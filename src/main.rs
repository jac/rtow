use std::io::{self, Write};
mod colour;
mod vec3;
mod ray;

use vec3::{Colour, Vec3, Point3};
use ray::Ray;

fn hit_sphere(center: &Point3, radius: f64, ray: &Ray) -> bool {
    // Quadratic formula
    let oc = ray.origin - *center;
    let a = Vec3::dot(ray.direction, ray.direction);
    let b = 2.0 * Vec3::dot(oc, ray.direction);
    let c = Vec3::dot(oc, oc) - radius * radius;
    let discriminant = b*b - 4.0 * a * c;
    discriminant > 0.0
}

fn ray_colour(ray: &Ray) -> Colour {
    if hit_sphere(&Point3::from((0.0, 0.0, -1.0)), 0.5, &ray) {
        Colour::from((1.0, 0.0, 0.0))
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

    let aspect_ratio = 16.0/ 9.0;
    let image_width = 384;
    let image_height = (image_width as f64 / aspect_ratio) as u32;

    // PPM header
    let header = format!("P3\n{} {}\n255\n", image_width, image_height);
    handle.write_all(header.as_bytes()).expect("Failed to write header");

    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::new();
    let horizontal = Vec3::from((viewport_width, 0.0, 0.0));
    let vertical = Vec3::from((0.0, viewport_height, 0.0));
    let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - Vec3::from((0.0, 0.0, focal_length));

    // Image content
    for j in (0..=image_height-1).rev() {
        for i in 0..image_width {
            let u = (i as f64) / ((image_width-1) as f64);
            let v = (j as f64) / ((image_height-1) as f64);
            
            let ray = Ray::new(origin, lower_left_corner + u * horizontal + v * vertical - origin);

            let pixel_colour = ray_colour(&ray);
            colour::write_colour(&mut handle, pixel_colour);
        }
    }
    
}
