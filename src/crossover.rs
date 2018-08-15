use bit_vec::BitVec;

/// Breed 2 parents to give an offspring
/// Method used is to randomly select a gene (locus) from each as the child
/// gene.
pub fn rand_interlace(parent1: &BitVec, parent2: &BitVec) -> BitVec {
  // will return a child of the longest length of the two parent chromosones
  let a: &BitVec;
  let b = if parent1.len() >= parent2.len() {
    a = parent1;
    parent2
  } else {
    a = parent2;
    parent1
  };

  a.iter()
    .zip(b.iter().cycle())
    .map(|(x, y)| if super::rand_bool() { x } else { y })
    .collect()
}
