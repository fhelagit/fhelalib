
use std::fmt::{self, Display};
use std::str::FromStr;
use crate::math::polynomial::polynomial::Polynomial;
extern crate serde_json;

#[cfg(test)]
use proptest::prelude::*;

use crate::tfhe::schemas::{TFHE_test_medium_u64, TFHE_test_small_u64, LWE_CT_Params, LWE_Params, GLWE_Params, from_u64, TFHESchema};


#[derive(Debug, PartialEq)]
pub struct GGSWCiphertext<S: TFHESchema, P: LWE_CT_Params<S>>(P::ContainerType);

impl<S: TFHESchema, P: LWE_CT_Params<S>> GGSWCiphertext<S, P>
{
    // fn new(data: Box<[u64; Polynomialsize*Masksize]>) -> Self
    // where
    //   [(); Masksize+1]: Sized
    // {
    //   GLWECiphertext(data)
    // }

    pub fn from_polynomial_list(data: P::ContainerType) -> Self {
        GGSWCiphertext(data)
    }

    pub fn get_poly_by_index(&self, ind: usize) -> Polynomial<{P::POLINOMIAL_SIZE}>{

        let mut v: Vec<u64> = Vec::with_capacity(P::POLINOMIAL_SIZE);
        for i in 0..P::POLINOMIAL_SIZE{
            v.push(from_u64::to(self.0[ind*P::POLINOMIAL_SIZE+i]));
        }
        Polynomial::<{P::POLINOMIAL_SIZE}>::new(v)



    }
}

impl<S: TFHESchema, P: LWE_CT_Params<S>> Display
    for GGSWCiphertext<S, P>
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

impl<S: TFHESchema, P: LWE_CT_Params<S>> FromStr
    for GGSWCiphertext<S, P>
{
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let data: P::ContainerType = serde_json::from_str(s).unwrap();
        Ok(GGSWCiphertext::from_polynomial_list(data))
    }
}

#[test]
fn test_ggsw_to_str_serialization() {
    // todo make iterative, make random
    let a = [0; 96].to_vec();
    let ct: GGSWCiphertext<TFHE_test_small_u64, LWE_Params<TFHE_test_small_u64>> = GGSWCiphertext::from_polynomial_list(a);

    let serialized = ct.to_string();
    let deserialized: GGSWCiphertext<TFHE_test_small_u64, LWE_Params<TFHE_test_small_u64>> = FromStr::from_str(&serialized).unwrap();
    assert_eq!(ct.0, deserialized.0);
}

#[cfg(test)]
proptest! {
    #![proptest_config(ProptestConfig::with_cases(1000))]
    #[test]
    fn pt_ggsw_ct_str_serialization(ct in any::<[u64; GLWE_Params::<TFHE_test_medium_u64>::POLINOMIAL_SIZE*(GLWE_Params::<TFHE_test_medium_u64>::MASK_SIZE+1)]>()
        .prop_map(|v| GGSWCiphertext::<TFHE_test_medium_u64, GLWE_Params<TFHE_test_medium_u64>>::from_polynomial_list(v.to_vec()))) {

        let serialized = ct.to_string();
        let deserialized: GGSWCiphertext<TFHE_test_medium_u64, GLWE_Params<TFHE_test_medium_u64>> = FromStr::from_str(&serialized).unwrap();
        prop_assert_eq!(ct, deserialized);

    }
}