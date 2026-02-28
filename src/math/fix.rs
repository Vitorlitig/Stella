// src/math/fix.rs

use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};
use serde::{Deserialize, Serialize};

const FRACTIONAL_BITS: usize = 32;
const ONE_RAW: i64 = 1 << FRACTIONAL_BITS;

/// Fixed-Point Number in Q32.32 format.
/// Guarantees bit-perfect determinism across platforms by avoiding IEEE 754 floating point.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Serialize, Deserialize)]
pub struct Q32(pub i64);

impl Q32 {
    pub const ZERO: Self = Q32(0);
    pub const ONE: Self = Q32(ONE_RAW);

    #[inline]
    pub const fn from_raw(raw: i64) -> Self {
        Self(raw)
    }

    #[inline]
    pub const fn from_i64(i: i64) -> Self {
        Self(i << FRACTIONAL_BITS)
    }

    #[inline]
    pub fn from_f64(f: f64) -> Self {
        Self((f * (1i64 << FRACTIONAL_BITS) as f64).round() as i64)
    }

    #[inline]
    pub fn to_f64(self) -> f64 {
        self.0 as f64 / (1i64 << FRACTIONAL_BITS) as f64
    }

    #[inline]
    pub fn max(self, other: Self) -> Self {
        if self.0 > other.0 {
            self
        } else {
            other
        }
    }

    #[inline]
    pub fn min(self, other: Self) -> Self {
        if self.0 < other.0 {
            self
        } else {
            other
        }
    }
}

impl Add for Q32 {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self {
        Self(self.0.checked_add(rhs.0).expect("Q32 addition overflow"))
    }
}

impl AddAssign for Q32 {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Sub for Q32 {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self {
        Self(self.0.checked_sub(rhs.0).expect("Q32 subtraction overflow"))
    }
}

impl SubAssign for Q32 {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl Mul for Q32 {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Self) -> Self {
        // Cast to i128 to prevent overflow during intermediate multiplication
        let a = self.0 as i128;
        let b = rhs.0 as i128;
        let result = (a * b) >> FRACTIONAL_BITS;
        Self(result.try_into().expect("Q32 multiplication overflow"))
    }
}

impl MulAssign for Q32 {
    #[inline]
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl Div for Q32 {
    type Output = Self;

    #[inline]
    fn div(self, rhs: Self) -> Self {
        assert_ne!(rhs.0, 0, "Q32 division by zero");
        // Shift left before division to maintain fractional precision
        let a = (self.0 as i128) << FRACTIONAL_BITS;
        let b = rhs.0 as i128;
        Self((a / b).try_into().expect("Q32 division overflow"))
    }
}

impl DivAssign for Q32 {
    #[inline]
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fixed_point_precision() {
        // Basic arithmetic constraints
        let a = Q32::from_f64(1.0);
        let b = Q32::from_f64(2.0);
        assert_eq!(a + b, Q32::from_f64(3.0));
        assert_eq!(b - a, Q32::from_f64(1.0));

        // Fractional multiplication
        let c = Q32::from_f64(1.5);
        let d = Q32::from_f64(2.25);
        assert_eq!(c * d, Q32::from_f64(3.375));

        // Fractional division
        assert_eq!(d / c, Q32::from_f64(1.5));

        // Ensure bit-perfect integer bounds
        let e = Q32::from_i64(5);
        let f = Q32::from_i64(5);
        assert_eq!(e * f, Q32::from_i64(25));
    }
}
