pub trait TFHESchema
where 
  Self::ArrayType: Clone,
  Self::ArrayType: serde::ser::Serialize,
  Self::ArrayType: Sized,
  Self::ArrayType: serde::de::DeserializeOwned
  {
    const LWE_K: usize;
    const GLWE_N: usize;
    const GLWE_K: usize;
    const CT_MODULUS: u64;
    type ScalarType;
    type ArrayType;
}

#[derive(Debug, PartialEq)]
pub struct TFHE_test_small_u64;

impl TFHESchema for TFHE_test_small_u64 {
    const LWE_K: usize = 2;
    const GLWE_N: usize = 32;
    const GLWE_K: usize = 1;
    const CT_MODULUS: u64 = u64::MAX;
    type ScalarType = u64;
    type ArrayType = Vec<Self::ScalarType>;
}

#[derive(Debug, PartialEq)]
pub struct TFHE_test_medium_u64;

impl TFHESchema for TFHE_test_medium_u64 {
  const LWE_K: usize = 586;
  const GLWE_N: usize = 1024;
  const GLWE_K: usize = 1;
  const CT_MODULUS: u64 = u64::MAX;
  type ScalarType = u64;
  type ArrayType = Vec<Self::ScalarType>;
}
