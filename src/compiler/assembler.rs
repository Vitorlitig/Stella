// src/compiler/assembler.rs

use crate::layer::dense::Dense;
use crate::math::fix::Q32;
use crate::math::matrix::Matrix;
use alloc::vec;

/// Compiles a conditional branch where State A transitions to State B if the condition (input) is 1.
/// There are no jump instructions; the state vector simply "morphs" into the next basic block.
///
/// State Vector Representation: [Condition_Flag, Context_A, Context_B]
pub fn build_conditional_branch() -> Dense {
    // W = [
    //   [ 1.0,  0.0,  0.0 ], -> Condition stays constant for testing
    //   [-1.0,  1.0,  0.0 ], -> Context A = Context A - Condition (Drains A if Condition is 1)
    //   [ 1.0,  1.0,  0.0 ]  -> Context B = Condition AND Context A
    // ]
    let w = Matrix::from_vec(
        3,
        3,
        vec![
            Q32::from_f64(1.0),
            Q32::ZERO,
            Q32::ZERO,
            Q32::from_f64(-1.0),
            Q32::from_f64(1.0),
            Q32::ZERO,
            Q32::from_f64(1.0),
            Q32::from_f64(1.0),
            Q32::ZERO,
        ],
    );

    // B = [
    //   [ 0.0 ],
    //   [ 0.0 ],
    //   [-1.0 ] -> Bias for the Context B AND gate threshold
    // ]
    let b = Matrix::from_vec(3, 1, vec![Q32::ZERO, Q32::ZERO, Q32::from_f64(-1.0)]);

    Dense::new(w, b)
}

/// Compiles an infinite loop (oscillator).
/// Acts as a NOT gate continuously feeding back into itself.
///
/// State Vector Representation: [Oscillator_State]
pub fn build_oscillator() -> Dense {
    // W = [-1.0]
    // B = [ 1.0]
    // S_next = clamp_01(-1.0 * S_current + 1.0)
    let w = Matrix::from_vec(1, 1, vec![Q32::from_f64(-1.0)]);
    let b = Matrix::from_vec(1, 1, vec![Q32::from_f64(1.0)]);

    Dense::new(w, b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_conditional_branching() {
        let branch_vm = build_conditional_branch();

        // --- Case 1: Input 0. Should stay in Block A. ---
        let mut state_stay = Matrix::from_vec(
            3,
            1,
            vec![
                Q32::from_f64(0.0), // Condition Flag
                Q32::from_f64(1.0), // Context A Active
                Q32::from_f64(0.0), // Context B Inactive
            ],
        );

        state_stay = branch_vm.forward(&state_stay);

        assert_eq!(state_stay.get(0, 0), Q32::from_f64(0.0)); // Condition remains 0
        assert_eq!(state_stay.get(1, 0), Q32::from_f64(1.0)); // Block A remains Active (1.0)
        assert_eq!(state_stay.get(2, 0), Q32::from_f64(0.0)); // Block B remains Inactive (0.0)

        // --- Case 2: Input 1. Should morph state to Block B. ---
        let mut state_jump = Matrix::from_vec(
            3,
            1,
            vec![
                Q32::from_f64(1.0), // Condition Flag
                Q32::from_f64(1.0), // Context A Active
                Q32::from_f64(0.0), // Context B Inactive
            ],
        );

        state_jump = branch_vm.forward(&state_jump);

        assert_eq!(state_jump.get(0, 0), Q32::from_f64(1.0)); // Condition remains 1
        assert_eq!(state_jump.get(1, 0), Q32::ZERO); // Block A deactivates (0.0)
        assert_eq!(state_jump.get(2, 0), Q32::from_f64(1.0)); // Block B activates (1.0)
    }

    #[test]
    fn test_infinite_loop() {
        let oscillator = build_oscillator();
        let mut state = Matrix::from_vec(1, 1, vec![Q32::from_f64(0.0)]);

        // Cycle 1: 0.0 -> 1.0
        state = oscillator.forward(&state);
        assert_eq!(state.get(0, 0), Q32::from_f64(1.0));

        // Cycle 2: 1.0 -> 0.0
        state = oscillator.forward(&state);
        assert_eq!(state.get(0, 0), Q32::ZERO);

        // Cycle 3: 0.0 -> 1.0
        state = oscillator.forward(&state);
        assert_eq!(state.get(0, 0), Q32::from_f64(1.0));
    }
}
