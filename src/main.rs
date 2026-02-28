// src/main.rs
use std::env;
use std::fs::File;
use std::io::Read;
use std::process;

use stella_vm::layer::dense::Dense;
use stella_vm::math::fix::Q32;
use stella_vm::vm::engine::Vm;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: stella_vm <program.stella> [input_val1] [input_val2] ...");
        process::exit(1);
    }

    let file_path = &args[1];
    let mut file = File::open(file_path).unwrap_or_else(|e| {
        eprintln!("Failed to open file '{}': {}", file_path, e);
        process::exit(1);
    });

    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap_or_else(|e| {
        eprintln!("Failed to read file '{}': {}", file_path, e);
        process::exit(1);
    });

    let logic_core: Dense = bincode::deserialize(&buffer).unwrap_or_else(|e| {
        eprintln!("Failed to deserialize '.stella' file: {}", e);
        process::exit(1);
    });

    // Determine state size from the loaded logic core weight matrix columns
    let state_size = logic_core.weights.cols;
    let mut vm = Vm::new(state_size, logic_core);

    // Parse optional inputs
    let mut inputs = Vec::new();
    for arg in args.iter().skip(2) {
        let val: f64 = arg.parse().unwrap_or_else(|_| {
            eprintln!("Invalid input value: {}", arg);
            process::exit(1);
        });
        inputs.push(Q32::from_f64(val));
    }

    if !inputs.is_empty() {
        vm.write_io(&inputs);
    }

    // Safely determine how many nodes to display without exceeding state size bounds
    let display_count = state_size.min(inputs.len().max(4));

    println!("--- Stella VM Execution Started ---");
    println!("State Size: {}", state_size);
    println!(
        "Initial I/O State: {:?}",
        format_io(&vm.read_io(display_count))
    );

    // Propagate the continuous state
    let execution_cycles = 1000;
    vm.run(execution_cycles);

    println!("--- Stella VM Execution Completed ---");
    println!("Cycles Run: {}", execution_cycles);
    println!(
        "Final I/O State: {:?}",
        format_io(&vm.read_io(display_count))
    );
}

// Helper to map Q32 vectors to clean f64 vectors for printing
fn format_io(io_data: &[Q32]) -> Vec<f64> {
    io_data.iter().map(|q| q.to_f64()).collect()
}
