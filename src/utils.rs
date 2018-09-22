use rulinalg::matrix::Matrix;

pub fn sigmoid(y: f64) -> f64 {
  1f64 / (1f64 + (-y).exp())
}

pub fn matrix_from_vector(vector: &[f64]) -> Matrix<f64> {
  Matrix::new(vector.len(), 1, vector)
}

pub fn matrix_from_matrix(matrix: &(usize, usize, &[f64])) -> Matrix<f64> {
  Matrix::new(matrix.0, matrix.1, matrix.2)
}
#[cfg(test)]
mod tests {
  use super::*;
  use proptest::prelude::any;

  proptest!{

  #[test]
    fn test_sigmoid(s in any::<f64>()) {
      let x = sigmoid(s);
      assert!(0.5f64 < x || x < 1f64 );
    }
  }
}
