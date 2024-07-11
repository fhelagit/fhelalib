use std::fmt::{self, Display};
use std::str::FromStr;
extern crate serde_json;

#[cfg(test)]
use proptest::prelude::*;
// #[cfg(test)]
// use proptest_derive::Arbitrary;
use crate::tfhe::schemas::{TFHE_test_medium_u64, TFHE_test_small_u64, LWE_CT_Params, LWE_Params, GLWE_Params, };

#[derive(Debug, PartialEq)]
pub struct GLWECiphertext<P: LWE_CT_Params>(P::ContainerType);

impl<P: LWE_CT_Params> GLWECiphertext<P>
{
    // fn new(data: Box<[u64; Polynomialsize*Masksize]>) -> Self
    // where
    //   [(); Masksize+1]: Sized
    // {
    //   GLWECiphertext(data)
    // }

    pub fn from_polynomial_list(data: P::ContainerType) -> Self {
        GLWECiphertext(data)
    }
}

impl<P: LWE_CT_Params> Display
    for GLWECiphertext<P>

{
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(
            formatter,
            "{}",
            serde_json::to_string(&self.0).unwrap()
        )
        .unwrap();
        Ok(())
    }
}

impl<P: LWE_CT_Params> FromStr
    for GLWECiphertext<P>
{
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let data: P::ContainerType = serde_json::from_str(s).unwrap();
        Ok(GLWECiphertext::from_polynomial_list(data))
    }
}

#[test]
fn test_glwe_to_str_serialization() {
    // todo make iterative, make random
    let a = [0; 96].to_vec();
    let ct: GLWECiphertext<LWE_Params<TFHE_test_small_u64>> = GLWECiphertext::from_polynomial_list(a);

    let serialized = ct.to_string();
    let deserialized: GLWECiphertext<LWE_Params<TFHE_test_small_u64>> = FromStr::from_str(&serialized).unwrap();
    assert_eq!(ct.0, deserialized.0);
}

#[cfg(test)]
proptest! {
    #![proptest_config(ProptestConfig::with_cases(1000))]
    #[test]
    fn pt_glwe_ct_str_serialization(ct in any::<[u64; 1024*2]>().prop_map(|v| GLWECiphertext::<GLWE_Params<TFHE_test_medium_u64>>::from_polynomial_list(v.to_vec()))) {

        let serialized = ct.to_string();
        let deserialized: GLWECiphertext<GLWE_Params<TFHE_test_medium_u64>> = FromStr::from_str(&serialized).unwrap();
        prop_assert_eq!(ct, deserialized);

    }
}
