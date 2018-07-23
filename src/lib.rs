#![feature(extern_prelude)] 
pub mod crossover;
pub mod organism;
pub mod select_breeders;
extern crate rand;
extern crate bit_vec;
extern crate rayon;
#[cfg(test)]
#[macro_use] extern crate proptest;

use rand::prelude::*;
pub use organism::Organism;
pub use select_breeders::select_breeders;
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

    use proptest::prelude::*;

//playing this stragely works, which is weird for 2 reasons 1. rng is not seeded and not using v
    proptest! {
        #[test]
        fn test_rand64(_v in any::<u64>()) {
            prop_assert!(super::rand_f64() <= 1f64);
            prop_assert!(super::rand_f64() > 0f64);
        }
    }
}
