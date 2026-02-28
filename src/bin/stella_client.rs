// src/bin/stella_client.rs

use std::env;
use std::fs::File;
use std::io::Read;
use std::process;

// use stella_vm::math::fix::Q32;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 4 {
        eprintln!("Usage: stella_client <encode|decode> <program.key> <val0> <val1> ...");
        process::exit(1);
    }

    let mode = &args[1];
    let key_path = &args[2];

    let mut key_file = File::open(key_path).expect("Failed to open key file");
    let mut key_str = String::new();
    key_file.read_to_string(&mut key_str).unwrap();
    let mapping: Vec<usize> = key_str
        .trim()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();
    let padded_size = mapping.len();

    let mut inputs = vec![0.0; padded_size];
    for (i, arg) in args.iter().skip(3).enumerate() {
        if i < padded_size {
            inputs[i] = arg.parse().expect("Invalid float");
        }
    }

    if mode == "encode" {
        // We want logical N_i to go to physical mapping[i]
        let mut encoded = vec![0.0; padded_size];
        for i in 0..padded_size {
            encoded[mapping[i]] = inputs[i];
        }
        println!("--- Encoded Obfuscated Payload (Copy into VM) ---");
        println!(
            "{}",
            encoded
                .iter()
                .map(|f| f.to_string())
                .collect::<Vec<String>>()
                .join(" ")
        );
    } else if mode == "decode" {
        // We want physical mapping[i] to revert to logical N_i
        let mut decoded = vec![0.0; padded_size];
        for i in 0..padded_size {
            decoded[i] = inputs[mapping[i]];
        }
        println!("--- Decoded True Logical State ---");
        println!("{:?}", decoded);
    } else {
        eprintln!("Unknown mode. Use 'encode' or 'decode'.");
    }
}
