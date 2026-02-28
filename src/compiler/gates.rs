// src/compiler/gates.rs

use crate::layer::dense::Dense;
use crate::math::fix::Q32;
use crate::math::matrix::Matrix;
use alloc::vec;

/// Generates an AND gate layer.
/// w1 = 1.0, w2 = 1.0, b = -1.0
/// Only fires if both inputs are 1.0 (1.0 + 1.0 - 1.0 = 1.0)
pub fn and_gate() -> Dense {
    Dense::new(
        Matrix::from_vec(1, 2, vec![Q32::from_f64(1.0), Q32::from_f64(1.0)]),
        Matrix::from_vec(1, 1, vec![Q32::from_f64(-1.0)]),
    )
}

/// Generates an OR gate layer.
/// w1 = 1.0, w2 = 1.0, b = 0.0
/// Fires if at least one input is 1.0. Clamped at 1.0 if both are 1.0.
pub fn or_gate() -> Dense {
    Dense::new(
        Matrix::from_vec(1, 2, vec![Q32::from_f64(1.0), Q32::from_f64(1.0)]),
        Matrix::from_vec(1, 1, vec![Q32::ZERO]),
    )
}

/// Generates a NAND gate layer.
/// w1 = -1.0, w2 = -1.0, b = 2.0
/// Inverts the logic. Fires if inputs are not both 1.0.
pub fn nand_gate() -> Dense {
    Dense::new(
        Matrix::from_vec(1, 2, vec![Q32::from_f64(-1.0), Q32::from_f64(-1.0)]),
        Matrix::from_vec(1, 1, vec![Q32::from_f64(2.0)]),
    )
}

/// XOR requires a hidden state. This generates the hidden layer.
/// Node 0: OR gate
/// Node 1: NAND gate
pub fn xor_hidden_layer() -> Dense {
    Dense::new(
        Matrix::from_vec(
            2,
            2,
            vec![
                Q32::from_f64(1.0),
                Q32::from_f64(1.0), // OR
                Q32::from_f64(-1.0),
                Q32::from_f64(-1.0), // NAND
            ],
        ),
        Matrix::from_vec(
            2,
            1,
            vec![
                Q32::ZERO,          // OR Bias
                Q32::from_f64(2.0), // NAND Bias
            ],
        ),
    )
}

/// XOR output layer.
/// Acts as an AND gate combining the OR and NAND results from the hidden layer.
pub fn xor_output_layer() -> Dense {
    // Equivalent to AND gate
    Dense::new(
        Matrix::from_vec(1, 2, vec![Q32::from_f64(1.0), Q32::from_f64(1.0)]),
        Matrix::from_vec(1, 1, vec![Q32::from_f64(-1.0)]),
    )
}

/// Generates an Identity matrix of size N x N.
/// Used to preserve register state perfectly across execution cycles.
pub fn identity_matrix(size: usize) -> Matrix {
    let mut m = Matrix::zeros(size, size);
    for i in 0..size {
        m.set(i, i, Q32::ONE);
    }
    m
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_neuron_and_gate() {
        let gate = and_gate();

        let run = |a: f64, b: f64| -> f64 {
            let input = Matrix::from_vec(2, 1, vec![Q32::from_f64(a), Q32::from_f64(b)]);
            gate.forward(&input).get(0, 0).to_f64()
        };

        assert_eq!(run(0.0, 0.0), 0.0);
        assert_eq!(run(0.0, 1.0), 0.0);
        assert_eq!(run(1.0, 0.0), 0.0);
        assert_eq!(run(1.0, 1.0), 1.0);
    }

    #[test]
    fn test_neuron_xor_gate() {
        let hidden = xor_hidden_layer();
        let output = xor_output_layer();

        let run = |a: f64, b: f64| -> f64 {
            let input = Matrix::from_vec(2, 1, vec![Q32::from_f64(a), Q32::from_f64(b)]);
            let h_state = hidden.forward(&input);
            output.forward(&h_state).get(0, 0).to_f64()
        };

        assert_eq!(run(0.0, 0.0), 0.0);
        assert_eq!(run(0.0, 1.0), 1.0);
        assert_eq!(run(1.0, 0.0), 1.0);
        assert_eq!(run(1.0, 1.0), 0.0); // The critical failure point of linear models
    }

    #[test]
    fn test_register_storage() {
        // Can a neuron "hold" a value of 42 for 100 cycles without decaying?
        let identity = identity_matrix(1);
        let mut state = Matrix::from_vec(1, 1, vec![Q32::from_f64(42.0)]);

        // 100 cycles of pure propagation (linear subsystem, bypassing activation)
        for _ in 0..100 {
            state = &identity * &state;
        }

        assert_eq!(state.get(0, 0), Q32::from_f64(42.0));
    }
}
