#![feature(rust_2018_preview, use_extern_macros)]
pub mod crossover;
pub mod organism;
pub mod select_breeders;
use rand::prelude::*;

/// (0..1]
pub fn rand_f64() -> f64 {
    let mut rng = thread_rng();
    rng.gen()
}
/// 1 || 0
pub fn rand_bool() -> bool {
    let mut rng = thread_rng();
    rng.gen()
}
#[cfg(test)]
mod tests {

    use proptest;

    proptest::proptest! {
        #[test]
        fn test_rand64(v in any::<u64>()) {
            prop_assert!(v < 1);
        }
    }
}
