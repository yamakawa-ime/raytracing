pub fn random_double() -> f64 {
    rand::random_range(0.0..1.0)
}

pub fn random_double_range(min: f64, max: f64) -> f64 {
    rand::random_range(min..max)
}

pub fn random_int(min: usize, max: usize) -> usize {
    random_double_range(min as f64, (max + 1) as f64) as usize
}
