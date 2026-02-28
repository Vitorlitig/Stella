// src/compiler/obfuscator.rs

use crate::layer::dense::Dense;
use crate::math::matrix::Matrix;

/// Applies a linear basis transformation to obfuscate the weights and biases of a layer.
/// Given an invertible matrix P and its inverse P_inv:
/// W' = P_inv * W * P
/// B' = P_inv * B
///
/// The state vector itself must also be transformed when loaded: S' = P_inv * S
/// To read the state back, the observer must apply: S = P * S'
pub fn transform_basis(layer: &Dense, p: &Matrix, p_inv: &Matrix) -> Dense {
    assert_eq!(p.rows, p.cols, "Transformation matrix P must be square");
    assert_eq!(
        p_inv.rows, p_inv.cols,
        "Inverse transformation matrix P_inv must be square"
    );
    assert_eq!(p.rows, p_inv.rows, "P and P_inv dimensions must match");
    assert_eq!(
        layer.weights.rows, p.rows,
        "Layer dimensions must match transformation matrix"
    );

    // W' = P_inv * (W * P)
    let w_p = &layer.weights * p;
    let w_prime = p_inv * &w_p;

    // B' = P_inv * B
    let b_prime = p_inv * &layer.biases;

    Dense::new(w_prime, b_prime)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::fix::Q32;
    use alloc::vec;

    #[test]
    fn test_isomorphism() {
        // Original Layer: W = [[2.0, 0.0], [0.0, 3.0]], B = [[1.0], [2.0]]
        let w = Matrix::from_vec(
            2,
            2,
            vec![Q32::from_f64(2.0), Q32::ZERO, Q32::ZERO, Q32::from_f64(3.0)],
        );
        let b = Matrix::from_vec(2, 1, vec![Q32::from_f64(1.0), Q32::from_f64(2.0)]);
        let clean_layer = Dense::new(w, b);

        // Transformation Matrix P = [[1.0, 1.0], [0.0, 1.0]] (Shear)
        let p = Matrix::from_vec(
            2,
            2,
            vec![
                Q32::from_f64(1.0),
                Q32::from_f64(1.0),
                Q32::ZERO,
                Q32::from_f64(1.0),
            ],
        );

        // Inverse Transformation P_inv = [[1.0, -1.0], [0.0, 1.0]]
        let p_inv = Matrix::from_vec(
            2,
            2,
            vec![
                Q32::from_f64(1.0),
                Q32::from_f64(-1.0),
                Q32::ZERO,
                Q32::from_f64(1.0),
            ],
        );

        // Obfuscate the layer
        let obf_layer = transform_basis(&clean_layer, &p, &p_inv);

        // Original State S = [[5.0], [10.0]]
        let state = Matrix::from_vec(2, 1, vec![Q32::from_f64(5.0), Q32::from_f64(10.0)]);

        // Run Clean Process (Simulating continuous linear sub-system, ignoring activation for math proof)
        // S_new = W * S + B
        let expected_w_s = &clean_layer.weights * &state;
        let expected_clean_output = &expected_w_s + &clean_layer.biases;

        // Run Obfuscated Process
        // 1. Encode State: S' = P_inv * S
        let obf_state = &p_inv * &state;

        // 2. Execute on Obfuscated VM: S'_new = W' * S' + B'
        let obf_w_s = &obf_layer.weights * &obf_state;
        let obf_raw_output = &obf_w_s + &obf_layer.biases;

        // 3. Decode State: S_new = P * S'_new
        let decoded_output = &p * &obf_raw_output;

        // The internal representation `obf_raw_output` should be unrecognizable (noise)
        assert_ne!(obf_raw_output, expected_clean_output);

        // But the decoded result must perfectly match the clean execution
        assert_eq!(decoded_output, expected_clean_output);
    }
}
