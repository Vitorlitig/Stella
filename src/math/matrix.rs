// src/math/matrix.rs
use crate::math::fix::Q32;
use alloc::vec;
use alloc::vec::Vec;
use core::ops::{Add, Mul};
use serde::{Deserialize, Serialize};

#[cfg(feature = "std")]
use rayon::prelude::*;

/// 2D Matrix backed by a flat contiguous vector.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Matrix {
    pub rows: usize,
    pub cols: usize,
    pub data: Vec<Q32>,
}

impl Matrix {
    /// Initializes a matrix of specified dimensions filled with Q32::ZERO.
    pub fn zeros(rows: usize, cols: usize) -> Self {
        Self {
            rows,
            cols,
            data: vec![Q32::ZERO; rows * cols],
        }
    }

    /// Creates a matrix from a 1D vector. Panics if length does not match rows * cols.
    pub fn from_vec(rows: usize, cols: usize, data: Vec<Q32>) -> Self {
        assert_eq!(rows * cols, data.len(), "Matrix dimension mismatch");
        Self { rows, cols, data }
    }

    #[inline]
    pub fn get(&self, row: usize, col: usize) -> Q32 {
        self.data[row * self.cols + col]
    }

    #[inline]
    pub fn set(&mut self, row: usize, col: usize, val: Q32) {
        self.data[row * self.cols + col] = val;
    }

    /// Returns a transposed version of the matrix.
    pub fn transpose(&self) -> Self {
        let mut result = Self::zeros(self.cols, self.rows);
        for r in 0..self.rows {
            for c in 0..self.cols {
                result.set(c, r, self.get(r, c));
            }
        }
        result
    }
}

impl Add for &Matrix {
    type Output = Matrix;

    fn add(self, rhs: Self) -> Matrix {
        assert_eq!(
            self.rows, rhs.rows,
            "Matrix row dimensions do not match for addition"
        );
        assert_eq!(
            self.cols, rhs.cols,
            "Matrix col dimensions do not match for addition"
        );

        let data = self
            .data
            .iter()
            .zip(rhs.data.iter())
            .map(|(&a, &b)| a + b)
            .collect();

        Matrix {
            rows: self.rows,
            cols: self.cols,
            data,
        }
    }
}

impl Mul for &Matrix {
    type Output = Matrix;

    fn mul(self, rhs: Self) -> Matrix {
        assert_eq!(
            self.cols, rhs.rows,
            "Matrix dimensions do not match for multiplication"
        );

        let mut result = Matrix::zeros(self.rows, rhs.cols);

        #[cfg(feature = "std")]
        {
            let rhs_cols = rhs.cols;
            let self_cols = self.cols;

            result
                .data
                .par_chunks_mut(rhs_cols)
                .enumerate()
                .for_each(|(r, row_slice)| {
                    for c in 0..rhs_cols {
                        let mut sum = Q32::ZERO;
                        for k in 0..self_cols {
                            let a = self.data[r * self_cols + k];
                            let b = rhs.data[k * rhs_cols + c];
                            sum += a * b;
                        }
                        row_slice[c] = sum;
                    }
                });
        }

        #[cfg(not(feature = "std"))]
        {
            for r in 0..self.rows {
                for c in 0..rhs.cols {
                    let mut sum = Q32::ZERO;
                    for k in 0..self.cols {
                        sum += self.get(r, k) * rhs.get(k, c);
                    }
                    result.set(r, c, sum);
                }
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matrix_multiplication() {
        let a = Matrix::from_vec(
            2,
            2,
            vec![
                Q32::from_f64(1.0),
                Q32::from_f64(2.0),
                Q32::from_f64(3.0),
                Q32::from_f64(4.0),
            ],
        );

        let b = Matrix::from_vec(
            2,
            2,
            vec![
                Q32::from_f64(2.0),
                Q32::from_f64(0.0),
                Q32::from_f64(1.0),
                Q32::from_f64(2.0),
            ],
        );

        let c = Matrix::from_vec(
            2,
            2,
            vec![
                Q32::from_f64(1.0),
                Q32::from_f64(1.0),
                Q32::from_f64(1.0),
                Q32::from_f64(1.0),
            ],
        );

        let ab = &a * &b;
        assert_eq!(
            ab.data,
            vec![
                Q32::from_f64(4.0),
                Q32::from_f64(4.0),
                Q32::from_f64(10.0),
                Q32::from_f64(8.0),
            ]
        );

        let ab_c = &ab * &c;
        let bc = &b * &c;
        let a_bc = &a * &bc;

        assert_eq!(ab_c, a_bc);
    }
}
