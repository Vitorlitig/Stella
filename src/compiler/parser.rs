// src/compiler/parser.rs

use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

use crate::layer::dense::Dense;
use crate::math::fix::Q32;
use crate::math::matrix::Matrix;

/// Parses a human-readable continuous-state assembly script into a compiled Dense logic core.
pub fn parse_asm(source: &str) -> Result<Dense, String> {
    let mut lines = source
        .lines()
        .filter(|l| !l.trim().is_empty() && !l.trim().starts_with(';'));

    // Extract .SIZE
    let size_line = lines.next().ok_or("Empty source. Must start with .SIZE")?;
    let size_parts: Vec<&str> = size_line.trim().split_whitespace().collect();
    if size_parts.len() != 2 || size_parts[0] != ".SIZE" {
        return Err("First line must be .SIZE <N>".to_string());
    }
    let size: usize = size_parts[1]
        .parse()
        .map_err(|_| "Invalid size configuration")?;

    // Initialize W (Identity matrix to preserve unchanged registers) and B (Zeros)
    let mut w = Matrix::zeros(size, size);
    for i in 0..size {
        w.set(i, i, Q32::ONE);
    }
    let mut b = Matrix::zeros(size, 1);

    // Track which rows have had their Identity wiped by a strict logic gate
    let mut cleared_rows = vec![false; size];

    for (line_num, line) in source.lines().enumerate() {
        let text = line.split(';').next().unwrap().trim(); // Strip comments
        if text.is_empty() || text.starts_with(".SIZE") {
            continue;
        }

        let tokens: Vec<&str> = text
            .split(|c: char| c.is_whitespace() || c == ',')
            .filter(|s| !s.is_empty())
            .collect();
        let op = tokens[0];

        let parse_reg = |t: &str| -> Result<usize, String> {
            if !t.starts_with('N') {
                return Err(format!("Expected register starting with 'N', got '{}'", t));
            }
            let idx = t[1..]
                .parse::<usize>()
                .map_err(|_| format!("Invalid register format '{}'", t))?;
            if idx >= size {
                return Err(format!("Register {} out of bounds (Size: {})", t, size));
            }
            Ok(idx)
        };

        let mut ensure_cleared = |matrix: &mut Matrix, dest: usize, sz: usize| {
            if !cleared_rows[dest] {
                for c in 0..sz {
                    matrix.set(dest, c, Q32::ZERO);
                }
                cleared_rows[dest] = true;
            }
        };

        let add_w = |matrix: &mut Matrix, r: usize, c: usize, val: f64| {
            let cur = matrix.get(r, c).to_f64();
            matrix.set(r, c, Q32::from_f64(cur + val));
        };
        let add_b = |matrix: &mut Matrix, r: usize, val: f64| {
            let cur = matrix.get(r, 0).to_f64();
            matrix.set(r, 0, Q32::from_f64(cur + val));
        };

        match op {
            "MOV" => {
                if tokens.len() != 3 {
                    return Err(format!("Line {}: MOV requires 2 args", line_num + 1));
                }
                let dest = parse_reg(tokens[1])?;
                ensure_cleared(&mut w, dest, size);

                if tokens[2].starts_with('N') {
                    let src = parse_reg(tokens[2])?;
                    add_w(&mut w, dest, src, 1.0);
                } else {
                    let val: f64 = tokens[2]
                        .parse()
                        .map_err(|_| format!("Line {}: Invalid float", line_num + 1))?;
                    add_b(&mut b, dest, val);
                }
            }
            "ADD" | "SUB" | "AND" | "OR" | "NAND" => {
                if tokens.len() != 4 {
                    return Err(format!("Line {}: {} requires 3 args", line_num + 1, op));
                }
                let dest = parse_reg(tokens[1])?;
                let src1 = parse_reg(tokens[2])?;
                let src2 = parse_reg(tokens[3])?;

                ensure_cleared(&mut w, dest, size);

                match op {
                    "ADD" => {
                        add_w(&mut w, dest, src1, 1.0);
                        add_w(&mut w, dest, src2, 1.0);
                    }
                    "SUB" => {
                        add_w(&mut w, dest, src1, 1.0);
                        add_w(&mut w, dest, src2, -1.0);
                    }
                    "AND" => {
                        add_w(&mut w, dest, src1, 1.0);
                        add_w(&mut w, dest, src2, 1.0);
                        add_b(&mut b, dest, -1.0);
                    }
                    "OR" => {
                        add_w(&mut w, dest, src1, 1.0);
                        add_w(&mut w, dest, src2, 1.0);
                    }
                    "NAND" => {
                        add_w(&mut w, dest, src1, -1.0);
                        add_w(&mut w, dest, src2, -1.0);
                        add_b(&mut b, dest, 2.0);
                    }
                    _ => unreachable!(),
                }
            }
            "NOT" => {
                if tokens.len() != 3 {
                    return Err(format!("Line {}: NOT requires 2 args", line_num + 1));
                }
                let dest = parse_reg(tokens[1])?;
                let src = parse_reg(tokens[2])?;

                ensure_cleared(&mut w, dest, size);
                add_w(&mut w, dest, src, -1.0);
                add_b(&mut b, dest, 1.0);
            }
            "JMP" => {
                if tokens.len() != 3 {
                    return Err(format!("Line {}: JMP requires 2 args", line_num + 1));
                }
                let src_ctx = parse_reg(tokens[1])?;
                let dst_ctx = parse_reg(tokens[2])?;

                add_w(&mut w, src_ctx, src_ctx, -1.0);
                add_w(&mut w, dst_ctx, src_ctx, 1.0);
            }
            "JEQ" => {
                if tokens.len() != 4 {
                    return Err(format!("Line {}: JEQ requires 3 args", line_num + 1));
                }
                let cond_reg = parse_reg(tokens[1])?;
                let src_ctx = parse_reg(tokens[2])?;
                let dst_ctx = parse_reg(tokens[3])?;

                // Drain Source IF condition is active
                add_w(&mut w, src_ctx, cond_reg, -1.0);

                // FIX: JEQ is a combinatorial 1-cycle pulse! We MUST clear its identity mapping
                ensure_cleared(&mut w, dst_ctx, size);

                // 1-Cycle Pulse to Destination IF condition AND source are active
                add_w(&mut w, dst_ctx, cond_reg, 1.0);
                add_w(&mut w, dst_ctx, src_ctx, 1.0);
                add_b(&mut b, dst_ctx, -1.0);
            }
            "LATCH" => {
                if tokens.len() != 3 {
                    return Err(format!("Line {}: LATCH requires 2 args", line_num + 1));
                }
                let dest = parse_reg(tokens[1])?;
                let src = parse_reg(tokens[2])?;

                add_w(&mut w, dest, src, 1.0);
            }
            _ => {
                return Err(format!(
                    "Line {}: Unknown instruction '{}'",
                    line_num + 1,
                    op
                ))
            }
        }
    }

    Ok(Dense::new(w, b))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser_add() {
        let asm = "
        .SIZE 3
        ADD N2, N0, N1
        ";
        let core = parse_asm(asm).expect("Failed to parse");
        assert_eq!(core.weights.get(2, 0), Q32::from_f64(1.0));
        assert_eq!(core.weights.get(2, 1), Q32::from_f64(1.0));
        assert_eq!(core.weights.get(2, 2), Q32::ZERO);
        assert_eq!(core.biases.get(2, 0), Q32::ZERO);
    }

    #[test]
    fn test_parser_sub() {
        let asm = "
        .SIZE 3
        SUB N2, N0, N1
        ";
        let core = parse_asm(asm).expect("Failed to parse");
        assert_eq!(core.weights.get(2, 0), Q32::from_f64(1.0));
        assert_eq!(core.weights.get(2, 1), Q32::from_f64(-1.0));
        assert_eq!(core.biases.get(2, 0), Q32::ZERO);
    }

    #[test]
    fn test_parser_and_gate() {
        let asm = "
        .SIZE 3
        AND N2, N0, N1
        ";
        let core = parse_asm(asm).expect("Failed to parse");
        assert_eq!(core.weights.get(2, 0), Q32::from_f64(1.0));
        assert_eq!(core.biases.get(2, 0), Q32::from_f64(-1.0));
    }

    #[test]
    fn test_parser_jmp() {
        let asm = "
        .SIZE 2
        JMP N0, N1
        ";
        let core = parse_asm(asm).expect("Failed to parse");
        assert_eq!(core.weights.get(0, 0), Q32::ZERO);
        assert_eq!(core.weights.get(1, 0), Q32::from_f64(1.0));
        assert_eq!(core.weights.get(1, 1), Q32::from_f64(1.0));
    }

    #[test]
    fn test_parser_jeq() {
        let asm = "
        .SIZE 3
        JEQ N0, N1, N2
        ";
        let core = parse_asm(asm).expect("Failed to parse");

        // N1 (Src) = N1 - N0
        assert_eq!(core.weights.get(1, 1), Q32::from_f64(1.0));
        assert_eq!(core.weights.get(1, 0), Q32::from_f64(-1.0));

        // N2 (Dst) = N0 AND N1 (Combinatorial! Identity MUST be wiped)
        assert_eq!(core.weights.get(2, 0), Q32::from_f64(1.0));
        assert_eq!(core.weights.get(2, 1), Q32::from_f64(1.0));
        assert_eq!(core.weights.get(2, 2), Q32::ZERO); // <-- Fixed!
        assert_eq!(core.biases.get(2, 0), Q32::from_f64(-1.0));
    }
}
