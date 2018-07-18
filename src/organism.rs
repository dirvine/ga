use bit_vec::BitVec; 

#[derive(PartialEq, Clone)]
pub struct Organism {
  chromosomes: BitVec,
  fitness: f64,
}

impl Organism {

  pub fn new(chromosomes: BitVec, fitness: f64) -> Organism {
     Organism {
       chromosomes,
       fitness
     }
  }
  pub fn read_last_fitness(&self) -> f64 {
    self.fitness
  }

// TODO code actual fitness function 
  pub fn set_fitness(&mut self, fitness: f64) {
    self.fitness = fitness;
  }

  pub fn chromosomes(&self) -> & BitVec {
    &self.chromosomes
  }
}
