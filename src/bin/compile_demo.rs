// src/bin/compile_demo.rs

use std::env;
use std::fs::File;
use std::io::{Read, Write};
use std::process;
use std::time::{SystemTime, UNIX_EPOCH};

use stella_vm::compiler::obfuscator::transform_basis;
use stella_vm::compiler::parser::parse_asm;
use stella_vm::layer::dense::Dense;
use stella_vm::math::chaos::HenonMap;
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

    println!("--- Stella Chaotic Polymorphic Assembler ---");
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
        "Padding state vector with Rotational Chaos nodes to size: {}",
        padded_size
    );

    // Phase 2: Pad the Matrix with Junk Nodes (Irrational Rotational Chaos)
    let mut w_padded = Matrix::zeros(padded_size, padded_size);
    let mut b_padded = Matrix::zeros(padded_size, 1);

    // Copy the genuine logic core into the padded matrix
    for i in 0..orig_size {
        for j in 0..orig_size {
            w_padded.set(i, j, clean_core.weights.get(i, j));
        }
        b_padded.set(i, 0, clean_core.biases.get(i, 0));
    }

    // Weave the Rotational Chaos Engine into the remaining padded space
    let mut i = orig_size;
    let mut angle: f64 = 1.0; // Start with 1.0 radian (irrational in degrees)

    while i < padded_size {
        if i + 1 < padded_size {
            // Pair up two junk nodes to create a 2D rotation matrix
            let cos_t = angle.cos();
            let sin_t = angle.sin();

            // Apply Affine Rotation centered around (0.5, 0.5)
            w_padded.set(i, i, Q32::from_f64(cos_t));
            w_padded.set(i, i + 1, Q32::from_f64(-sin_t));
            w_padded.set(i + 1, i, Q32::from_f64(sin_t));
            w_padded.set(i + 1, i + 1, Q32::from_f64(cos_t));

            // Bias adjustments to center the rotation at 0.5, plus a small chaotic kick
            // to ensure nodes starting at 0.0 immediately fly into orbit.
            let b_x = 0.5 - 0.5 * cos_t + 0.5 * sin_t + 0.123;
            let b_y = 0.5 - 0.5 * sin_t - 0.5 * cos_t + 0.321;

            b_padded.set(i, 0, Q32::from_f64(b_x));
            b_padded.set(i + 1, 0, Q32::from_f64(b_y));

            // Rotate the angle by the Golden Ratio conjugate to ensure no two pairs orbit identically
            angle += 0.6180339887;
            i += 2;
        } else {
            // If there is an odd node left over, map it to a simple bouncing oscillator
            w_padded.set(i, i, Q32::from_f64(-1.0)); // Flip state
            b_padded.set(i, 0, Q32::from_f64(1.0)); // 0.0 -> 1.0 -> 0.0 -> 1.0
            i += 1;
        }
    }

    let padded_core = Dense::new(w_padded, b_padded);

    // Phase 3: Cryptographic Shuffle via Hénon Map Chaos
    let mut indices: Vec<usize> = (0..padded_size).collect();

    // Seed the Hénon Map using high-precision time entropy mapped to [-1.0, 1.0]
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .subsec_nanos() as f64;
    let seed_x = (nanos / 1_000_000_000.0) * 2.0 - 1.0;
    let seed_y = 0.0;

    println!(
        "Initializing Hénon Strange Attractor with seed X: {:.6}",
        seed_x
    );
    let mut chaos = HenonMap::new(seed_x, seed_y);

    // Fisher-Yates Shuffle driven by the chaotic attractor
    for i in (1..padded_size).rev() {
        let j = chaos.next_usize() % (i + 1);
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

    println!("Success! State space encrypted via non-linear chaos.");
    println!("Executable: {}", out_stella);
    println!("Private Key: {}", out_key);
}
