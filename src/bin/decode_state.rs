// src/bin/decode_state.rs
use std::env;
use std::process;

use stella_vm::math::fix::Q32;
use stella_vm::math::matrix::Matrix;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: decode_state <state_size> <val1> <val2> ...");
        process::exit(1);
    }

    let size: usize = args[1].parse().unwrap_or_else(|_| {
        eprintln!("Invalid state size");
        process::exit(1);
    });

    if args.len() - 2 != size {
        eprintln!("Expected {} state values, got {}", size, args.len() - 2);
        process::exit(1);
    }

    let mut obf_state_data = Vec::new();
    for arg in args.iter().skip(2) {
        let val: f64 = arg.parse().unwrap_or_else(|_| {
            eprintln!("Invalid state value: {}", arg);
            process::exit(1);
        });
        obf_state_data.push(Q32::from_f64(val));
    }

    let obf_state = Matrix::from_vec(size, 1, obf_state_data);

    // Re-generate the exact Permutation matrix P used during compilation
    let mut p = Matrix::zeros(size, size);
    for i in 0..size {
        let mapped = (i + 1) % size;
        p.set(i, mapped, Q32::from_f64(1.0));
    }

    // Decode: Clean State = P * Obfuscated State
    let clean_state = &p * &obf_state;

    let formatted: Vec<f64> = clean_state.data.iter().map(|q| q.to_f64()).collect();
    println!("--- Stella VM State Decoder ---");
    println!("Obfuscated Input: {:?}", args[2..].to_vec());
    println!("Decoded True Logic State: {:?}", formatted);
}
