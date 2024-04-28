use rand::{CryptoRng, Rng};


pub fn random_array<R: CryptoRng + Rng>(n: usize, min: i32, max: i32) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    (0..n).map(|_| rng.gen_range(min..max)).collect()
}