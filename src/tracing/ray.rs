use super::{Point3, Vec3};

pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Self {
        Ray { origin, direction }
    }

    // P(t)= A + t * b
    // A: Origin; b: Direction; t: Distance from Origin in Direction
    pub fn at(&self, t: f64) -> Point3 {
        self.origin + t * self.direction
    }
}
