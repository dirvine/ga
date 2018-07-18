#![feature(rust_2018_preview, use_extern_macros)]
pub mod organism;
pub mod weighted_choice;
pub mod crossover;
use rand::prelude::*;

/// (0..1]
 pub fn rand_f64() -> f64 {
    let mut rng = thread_rng();
    rng.gen()
}
/// 1 || 0 
 pub fn rand_bool() ->bool  {
    let mut rng = thread_rng();
    rng.gen()
}
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
