//! Stochastic universal sampling algorithm:
//! https://en.wikipedia.org/wiki/Stochastic_universal_sampling

use super::organism::Organism;
#[cfg(test)]
use super::rand_f64;
use rand::{thread_rng, Rng};
use rayon::prelude::*;

    /// SUS is a development of fitness proportionate selection (FPS) which exhibits
    /// no bias and minimal spread. Where FPS chooses several solutions from the population
    /// by repeated random sampling, SUS uses a single random value to sample all of the
    /// solutions by choosing them at evenly spaced intervals. This gives weaker members
    /// of the population (according to their fitness) a chance to be chosen and thus
    /// reduces the unfair nature of fitness-proportional selection methods.
    /// see https://en.wikipedia.org/wiki/Stochastic_universal_sampling for details
    pub fn select_breeders(
        population: &[Organism],
        num_to_select: usize,
    ) -> Vec<Organism> {
        let total_fitness: f64 = population.par_iter().map(|x| x.read_last_fitness()).sum();
        let dist_between_pointers: f64 = total_fitness / num_to_select as f64;

        let mut points: Vec<f64> = Vec::with_capacity(num_to_select);

        for i in 0..num_to_select {
            points.push(
                thread_rng().gen_range(0f64, dist_between_pointers)
                    + (i as f64 * dist_between_pointers),
            );
        }
        roulette_wheel_selection(population, &mut points)
    }

    fn roulette_wheel_selection(
        population: &[Organism],
        points: &mut [f64],
    ) -> Vec<Organism> {
        let mut keep: Vec<Organism> = Vec::with_capacity(points.len());
        let mut accumulated_fitness = 0f64;
        points.sort_by(|a, b| a.partial_cmp(b).unwrap()); // sort floats decending
        
        // population.sort_by(|a, b| {
        //     b.read_last_fitness()
        //         .partial_cmp(&a.read_last_fitness())
        //         .unwrap()
        // }); // sort floats ascending
        for p in points {
            let mut i: usize = 0;
            while accumulated_fitness <= *p && i < population.len() - 1 {
                accumulated_fitness += population[i].read_last_fitness();
                i += 1;
            }
            accumulated_fitness = 0f64; // This is a slight "improvment" on the RWS algo
            keep.push(population[i].clone());
        }
        keep
    }

#[cfg(test)]
mod tests {
    use bit_vec::BitVec;
    use super::Organism;
    use super::select_breeders;
    use super::rand_f64;


fn create_population(num: usize) -> Vec<Organism> {
        let bv = BitVec::from_elem(num, true);
        let mut population = Vec::with_capacity(num);
        for _ in 0..num {
            population.push(Organism::new(bv.clone(), rand_f64()));
        }
        population
}
    #[test]
    fn check_n_populations() {
       let num = 20;
       let mut population = create_population(num);
        let new = select_breeders(&mut population, num);
        assert_eq!(num, new.len());
        for i in 0..new.len() {
            println!(
                "population {} new {}",
                population[i].read_last_fitness(),
                new[i].read_last_fitness()
            )
        }
        let new = select_breeders(&mut population, num / 2);
        assert_eq!(num / 2, new.len());
        let new = select_breeders(&mut population, num * 2);
        assert_eq!(num * 2, new.len());
    }

}
