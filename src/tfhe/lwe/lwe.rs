use std::fmt::{self, Display};
use std::str::FromStr;
extern crate serde_json;

#[cfg(test)]
use proptest::prelude::*;
#[cfg(test)]
use proptest_derive::Arbitrary;

use Vec;

use crate::tfhe::schemas::{TFHESchema, TFHE_test_medium_u64, TFHE_test_small_u64};

#[derive(Debug, PartialEq)]
pub struct LWECiphertext<S: TFHESchema>(S::ArrayType);

impl<S: TFHESchema> LWECiphertext<S> {
    // fn new(data: Box<[u64; Polynomialsize*Masksize]>) -> Self
    // where
    //   [(); Masksize+1]: Sized
    // {
    //   GLWECiphertext(data)
    // }

    pub fn from_polynomial_list(data: S::ArrayType) -> Self {
        LWECiphertext(data)
    }
}

impl<S: TFHESchema> Display for LWECiphertext<S>
{
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "{}", serde_json::to_string(&self.0).unwrap()).unwrap();
        Ok(())
    }
}

impl<S: TFHESchema> FromStr for LWECiphertext<S>
{
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let data = serde_json::from_str(s).unwrap();
        Ok(LWECiphertext::from_polynomial_list(data))
    }
}

#[test]
fn test_lwe_to_str_serialization() {
    // todo make iterative, make random
    let a = [5; 96].to_vec();
    let ct: LWECiphertext<TFHE_test_small_u64> = LWECiphertext::from_polynomial_list(a);

    let serialized = ct.to_string();
    let deserialized: LWECiphertext<TFHE_test_small_u64> = FromStr::from_str(&serialized).unwrap();
    assert_eq!(ct.0, deserialized.0);
}

#[cfg(test)]
proptest! {
    #![proptest_config(ProptestConfig::with_cases(1000))]
    #[test]
    fn pt_glwe_ct_str_serialization(ct in any::<[u64; 587]>().prop_map(|v| LWECiphertext::<TFHE_test_medium_u64>::from_polynomial_list(v.to_vec()))) {

        let serialized = ct.to_string();
        let deserialized: LWECiphertext<TFHE_test_medium_u64> = FromStr::from_str(&serialized).unwrap();
        prop_assert_eq!(ct, deserialized);

    }
}
