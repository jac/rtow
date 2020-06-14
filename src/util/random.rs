use rand::{self, Rng};

pub fn random() -> f64 {
    random_range(0.0, 1.0)
}

pub fn random_range(min: f64, max: f64) -> f64 {
    rand::thread_rng().gen_range(min, max)
}
