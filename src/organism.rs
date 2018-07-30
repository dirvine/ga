use bit_vec::BitVec;

#[derive(Fail, Debug, PartialEq, Clone)]
#[fail(
  display = "An error occurred, chromeosones  {:?} : fitness  ({})",
  chromosomes,
  fitness
)]
pub struct Organism {
  chromosomes: BitVec,
  fitness: f64,
}

impl Organism {
  pub fn new(chromosomes: BitVec, fitness: f64) -> Organism {
    Organism {
      chromosomes,
      fitness,
    }
  }
  /// Getter (immutable)
  pub fn read_last_fitness(&self) -> f64 {
    self.fitness
  }

/// Setter
  // TODO code actual fitness function
  pub fn set_fitness(&mut self, fitness: f64) {
    self.fitness = fitness;
  }

/// Getter  (immutable)
  pub fn chromosomes(&self) -> &BitVec {
    &self.chromosomes
  }
}
