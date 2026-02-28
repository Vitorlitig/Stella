// src/lib.rs
#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

pub mod compiler;
pub mod layer;
pub mod math;
pub mod vm;
