use rand::Rng;

pub fn add_one(x: i32) -> i32 {
    x + 1
}

pub fn random_number() -> i32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(1..=10)
}