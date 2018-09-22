#[macro_use]
extern crate criterion;

use bit_vec::BitVec;
use criterion::Criterion;
use ga::rand_f64;
use ga::select_breeders;
use ga::Organism;
fn create_population(num: usize) -> Vec<Organism> {
    let bv = BitVec::from_elem(num, true);
    let mut population = Vec::with_capacity(num);
    for _ in 0..num {
        population.push(Organism::new(bv.clone(), rand_f64()));
    }
    population
}

fn bench_selection(c: &mut Criterion) {
    let population = create_population(20);
    c.bench_function("bench_selection", move |b| {
        b.iter(|| select_breeders(&population, 10))
    });;

}
criterion_group!(benches, bench_selection);
criterion_main!(benches);
