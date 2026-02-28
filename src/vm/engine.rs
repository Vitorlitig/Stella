// src/vm/engine.rs

use crate::layer::dense::Dense;
use crate::math::fix::Q32;
use crate::math::matrix::Matrix;
use alloc::vec::Vec;

/// The Stella Virtual Machine Runtime.
/// Executes the continuous-state neural representation of discrete assembly logic.
pub struct Vm {
    /// The memory of the machine, represented as a single massive continuous vector.
    pub state: Matrix,
    /// The compiled logic of the program, represented as a single neural layer.
    pub logic_core: Dense,
}

impl Vm {
    /// Initializes the VM with a compiled logic core and an empty state vector.
    /// `state_size`: The total number of neurons (I/O + Hidden State).
    pub fn new(state_size: usize, logic_core: Dense) -> Self {
        assert_eq!(
            state_size, logic_core.weights.cols,
            "State size must match logic core input dimensions"
        );
        assert_eq!(
            state_size, logic_core.weights.rows,
            "Logic core must map state back to same dimensions to loop"
        );

        Self {
            state: Matrix::zeros(state_size, 1),
            logic_core,
        }
    }

    /// Injects external data into the I/O mapped region of the state vector (the first N neurons).
    pub fn write_io(&mut self, inputs: &[Q32]) {
        assert!(
            inputs.len() <= self.state.rows,
            "I/O payload exceeds state vector capacity"
        );
        for (i, &val) in inputs.iter().enumerate() {
            self.state.set(i, 0, val);
        }
    }

    /// Reads the output from the I/O mapped region of the state vector.
    pub fn read_io(&self, count: usize) -> Vec<Q32> {
        assert!(
            count <= self.state.rows,
            "Requested I/O read exceeds state vector capacity"
        );
        let mut outputs = Vec::with_capacity(count);
        for i in 0..count {
            outputs.push(self.state.get(i, 0));
        }
        outputs
    }

    /// Executes a single clock cycle.
    /// Mathematically: S_new = Clamp(W * S_old + B)
    pub fn step(&mut self) {
        self.state = self.logic_core.forward(&self.state);
    }

    /// Propagates the system continuously for a designated number of clock cycles.
    pub fn run(&mut self, cycles: usize) {
        for _ in 0..cycles {
            self.step();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;

    #[test]
    fn test_vm_execution_loop() {
        // Build a mock 2-neuron program.
        // Neuron 0: Oscillates (NOT gate of itself).
        // Neuron 1: Copies Neuron 0 (Identity mapping of Neuron 0).

        // W = [
        //   [-1.0,  0.0 ], -> N0 = NOT N0
        //   [ 1.0,  0.0 ]  -> N1 = N0
        // ]
        let w = Matrix::from_vec(
            2,
            2,
            vec![
                Q32::from_f64(-1.0),
                Q32::ZERO,
                Q32::from_f64(1.0),
                Q32::ZERO,
            ],
        );

        // B = [
        //   [ 1.0 ], -> Bias for NOT gate
        //   [ 0.0 ]
        // ]
        let b = Matrix::from_vec(2, 1, vec![Q32::from_f64(1.0), Q32::ZERO]);

        let compiled_core = Dense::new(w, b);
        let mut vm = Vm::new(2, compiled_core);

        // Verify initial state is 0.0
        assert_eq!(vm.read_io(2), vec![Q32::ZERO, Q32::ZERO]);

        // Cycle 1
        // N0 (prev 0.0) -> NOT -> 1.0
        // N1 (prev 0.0) -> N0  -> 0.0 (uses N0's previous state)
        vm.step();
        assert_eq!(vm.read_io(2), vec![Q32::from_f64(1.0), Q32::ZERO]);

        // Cycle 2
        // N0 (prev 1.0) -> NOT -> 0.0
        // N1 (prev 0.0) -> N0  -> 1.0 (uses N0's previous state)
        vm.step();
        assert_eq!(vm.read_io(2), vec![Q32::ZERO, Q32::from_f64(1.0)]);

        // Cycle 3
        // N0 (prev 0.0) -> NOT -> 1.0
        // N1 (prev 1.0) -> N0  -> 0.0 (uses N0's previous state)
        vm.step();
        assert_eq!(vm.read_io(2), vec![Q32::from_f64(1.0), Q32::ZERO]);

        // Let it run for 100 continuous cycles and assert deterministic predictable state
        vm.run(100);
        // After an even number of total cycles (103 total run), the state should match cycle 1 (odd)
        assert_eq!(vm.read_io(2), vec![Q32::from_f64(1.0), Q32::ZERO]);
    }
}
