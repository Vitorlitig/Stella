// src/bin/compile_demo.rs

use std::env;
use std::fs::File;
use std::io::{Read, Write};
use std::process;
use std::time::{SystemTime, UNIX_EPOCH};

use stella_vm::compiler::obfuscator::transform_basis;
use stella_vm::compiler::parser::parse_asm;
use stella_vm::layer::dense::Dense;
use stella_vm::math::fix::Q32;
use stella_vm::math::matrix::Matrix;

const TARGET_STATE_SIZE: usize = 16; // Pad all programs to 16 nodes to defeat brute-force

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: compile_demo <source.asm> [output_prefix]");
        process::exit(1);
    }

    let input_path = &args[1];
    let output_prefix = if args.len() >= 3 { &args[2] } else { "out" };
    let out_stella = format!("{}.stella", output_prefix);
    let out_key = format!("{}.key", output_prefix);

    println!("--- Stella Polymorphic Assembler ---");
    let mut file = File::open(input_path).expect("Failed to open ASM file");
    let mut source = String::new();
    file.read_to_string(&mut source)
        .expect("Failed to read ASM file");

    // Phase 1: Synthesize Clean Core
    let clean_core = parse_asm(&source).expect("Assembly Syntax Error");
    let orig_size = clean_core.weights.cols;
    println!("Parsed ASM. Original State Size: {}", orig_size);

    let padded_size = orig_size.max(TARGET_STATE_SIZE);
    println!(
        "Padding state vector with junk nodes to size: {}",
        padded_size
    );

    // Phase 2: Pad the Matrix with Junk Nodes (Identity mappings)
    let mut w_padded = Matrix::zeros(padded_size, padded_size);
    let mut b_padded = Matrix::zeros(padded_size, 1);

    for i in 0..padded_size {
        if i < orig_size {
            for j in 0..orig_size {
                w_padded.set(i, j, clean_core.weights.get(i, j));
            }
            b_padded.set(i, 0, clean_core.biases.get(i, 0));
        } else {
            w_padded.set(i, i, Q32::ONE); // Junk nodes harmlessly circulate their own noise
        }
    }
    let padded_core = Dense::new(w_padded, b_padded);

    // Phase 3: Randomized Permutation Shuffle
    let mut indices: Vec<usize> = (0..padded_size).collect();

    // Simple PRNG seeded by time for the shuffle
    let mut seed = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos() as u64;
    let mut rng = || {
        seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
        seed
    };

    // Fisher-Yates Shuffle
    for i in (1..padded_size).rev() {
        let j = (rng() as usize) % (i + 1);
        indices.swap(i, j);
    }

    let mut p = Matrix::zeros(padded_size, padded_size);
    let mut p_inv = Matrix::zeros(padded_size, padded_size);
    for (i, &mapped) in indices.iter().enumerate() {
        p.set(i, mapped, Q32::from_f64(1.0));
        p_inv.set(mapped, i, Q32::from_f64(1.0));
    }

    // Phase 4: Obfuscate and Serialize
    let obf_core = transform_basis(&padded_core, &p, &p_inv);

    let encoded_core = bincode::serialize(&obf_core).expect("Serialization failed");
    File::create(&out_stella)
        .unwrap()
        .write_all(&encoded_core)
        .unwrap();

    // Phase 5: Export the Private Key (The mapping array)
    let key_data = indices
        .iter()
        .map(|i| i.to_string())
        .collect::<Vec<String>>()
        .join(",");
    File::create(&out_key)
        .unwrap()
        .write_all(key_data.as_bytes())
        .unwrap();

    println!("Success! 20 Trillion possible permutations generated.");
    println!("Executable: {}", out_stella);
    println!("Private Key: {}", out_key);
}
