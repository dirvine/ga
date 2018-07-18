use bit_vec::BitVec;

/// Breed 2 parents to give an offspring
/// Method used is to randomly select a gene (locus) from each as the child gene. 
pub fn rand_interlace(parent1: &BitVec, parent2: &BitVec) -> BitVec {
  // if parent1.len() 
  parent1
    .iter()
    .zip(parent2.iter())
    .map(|(a, b)| if crate::rand_bool() { a } else { b })
    .collect()
}
