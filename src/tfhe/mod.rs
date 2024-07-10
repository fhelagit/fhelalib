pub mod glwe;
pub mod lwe;
use crate::random;
pub mod schemas;
pub mod secret_key;

pub const GLWE_K: usize = 4;
pub const GLWE_N: usize = 256;
pub const GLWE_P: usize = 2;
pub const GLWE_Q: usize = 64;

pub const GLEV_L: usize = 3;
pub const GLEV_B: usize = 6;
