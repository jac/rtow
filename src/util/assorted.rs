use rand::Rng;

pub fn random() -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(0.0, 1.0)
}

pub fn random_double(min: f64, max: f64) -> f64 {
    min + (max - min) * random()
}

pub fn clamp(val: f64, min: f64, max: f64) -> f64 {
    val.min(max).max(min)
}
