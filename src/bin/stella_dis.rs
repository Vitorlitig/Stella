// src/bin/stella_dis.rs
use std::env;
use std::fs::File;
use std::io::Read;
use std::process;

use stella_vm::compiler::obfuscator::transform_basis;
use stella_vm::compiler::parser::parse_asm;
use stella_vm::layer::dense::Dense;
use stella_vm::math::fix::Q32;
use stella_vm::math::matrix::Matrix;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: stella_dis <original.asm> <obfuscated.stella>");
        process::exit(1);
    }

    // 1. Load the "Ciphertext" (The .stella weights)
    let mut file = File::open(&args[2]).expect("Failed to open stella file");
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)
        .expect("Failed to read stella file");
    let target_core: Dense = bincode::deserialize(&buffer).expect("Deserialization failed");

    // 2. Load the "Plaintext" (The original ASM)
    let mut asm_file = File::open(&args[1]).expect("Failed to open ASM file");
    let mut source = String::new();
    asm_file
        .read_to_string(&mut source)
        .expect("Failed to read ASM file");
    let clean_core = parse_asm(&source).expect("Failed to parse clean ASM");

    let size = clean_core.weights.rows;
    println!("--- Stella Symbolic Disassembler ---");
    println!("Attempting to break {}x{} state obfuscation...", size, size);

    // 3. Brute force permutations (Limited to small sizes for demo)
    let perms = get_all_permutations(size);
    let mut found = false;

    for p_vec in perms {
        let (p, p_inv) = build_matrices_from_indices(&p_vec);
        let test_core = transform_basis(&clean_core, &p, &p_inv);

        if test_core.weights == target_core.weights && test_core.biases == target_core.biases {
            println!("\n[!] SUCCESS: Obfuscation key recovered!");
            println!("Mapping: {:?}", p_vec);
            found = true;
            break;
        }
    }

    if !found {
        println!("\n[-] FAILED: Could not recover the permutation key.");
    }
}

fn get_all_permutations(n: usize) -> Vec<Vec<usize>> {
    let mut res = Vec::new();
    let mut current = (0..n).collect::<Vec<_>>();
    generate_perms(n, &mut current, &mut res);
    res
}

fn generate_perms(n: usize, a: &mut Vec<usize>, res: &mut Vec<Vec<usize>>) {
    if n == 1 {
        res.push(a.clone());
        return;
    }
    for i in 0..n {
        generate_perms(n - 1, a, res);
        if n % 2 == 0 {
            a.swap(i, n - 1);
        } else {
            a.swap(0, n - 1);
        }
    }
}

fn build_matrices_from_indices(indices: &[usize]) -> (Matrix, Matrix) {
    let n = indices.len();
    let mut p = Matrix::zeros(n, n);
    let mut p_inv = Matrix::zeros(n, n);
    for (i, &mapped) in indices.iter().enumerate() {
        p.set(i, mapped, Q32::from_f64(1.0));
        p_inv.set(mapped, i, Q32::from_f64(1.0));
    }
    (p, p_inv)
}
