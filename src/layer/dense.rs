// src/layer/dense.rs
use crate::math::fix::Q32;
use crate::math::matrix::Matrix;
use serde::{Deserialize, Serialize};

/// Hard activation function: Clamps a continuous value between 0.0 and 1.0.
/// This acts as our "digital signal restoration", snapping noisy analog states back to discrete logic (0 or 1).
#[inline]
pub fn clamp_01(val: Q32) -> Q32 {
    val.max(Q32::ZERO).min(Q32::ONE)
}

/// Applies the activation function element-wise across a matrix.
pub fn activate(m: &Matrix) -> Matrix {
    let mut result = m.clone();
    for val in result.data.iter_mut() {
        *val = clamp_01(*val);
    }
    result
}

/// A standard Dense (Fully Connected) Neural Layer.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dense {
    pub weights: Matrix,
    pub biases: Matrix,
}

impl Dense {
    /// Constructs a new Dense layer.
    /// `weights`: An (N x M) matrix.
    /// `biases`: An (N x 1) column matrix.
    pub fn new(weights: Matrix, biases: Matrix) -> Self {
        assert_eq!(
            weights.rows, biases.rows,
            "Weights row count and biases row count must match"
        );
        assert_eq!(biases.cols, 1, "Biases must be an (N x 1) column vector");
        Self { weights, biases }
    }

    /// Computes the forward pass: Output = Activation(W * Input + B)
    /// `input`: An (M x 1) column matrix.
    pub fn forward(&self, input: &Matrix) -> Matrix {
        assert_eq!(
            self.weights.cols, input.rows,
            "Input rows must match weights columns"
        );
        assert_eq!(input.cols, 1, "Input must be an (M x 1) column vector");

        let w_x = &self.weights * input;
        let w_x_b = &w_x + &self.biases;

        activate(&w_x_b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;

    #[test]
    fn test_activation_function() {
        let neg = Q32::from_f64(-5.0);
        let zero = Q32::from_f64(0.0);
        let mid = Q32::from_f64(0.5);
        let one = Q32::from_f64(1.0);
        let over = Q32::from_f64(42.0);

        assert_eq!(clamp_01(neg), Q32::ZERO);
        assert_eq!(clamp_01(zero), Q32::ZERO);
        assert_eq!(clamp_01(mid), Q32::from_f64(0.5));
        assert_eq!(clamp_01(one), Q32::ONE);
        assert_eq!(clamp_01(over), Q32::ONE);
    }

    #[test]
    fn test_dense_forward() {
        let w = Matrix::from_vec(
            2,
            2,
            vec![
                Q32::from_f64(1.0),
                Q32::from_f64(0.5),
                Q32::from_f64(-1.0),
                Q32::from_f64(2.0),
            ],
        );

        let b = Matrix::from_vec(2, 1, vec![Q32::from_f64(0.0), Q32::from_f64(-0.5)]);

        let layer = Dense::new(w, b);

        let input = Matrix::from_vec(2, 1, vec![Q32::from_f64(1.0), Q32::from_f64(1.0)]);

        let output = layer.forward(&input);

        assert_eq!(output.rows, 2);
        assert_eq!(output.cols, 1);
        assert_eq!(output.get(0, 0), Q32::ONE);
        assert_eq!(output.get(1, 0), Q32::from_f64(0.5));
    }
}
