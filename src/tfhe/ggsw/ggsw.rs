use crate::math::polynomial::polynomial::{decompose_polynomial_assign, Polynomial};
use crate::tfhe::glwe::GLWECiphertext;
use std::fmt::{self, Display};
use std::ops;
use std::str::FromStr;
extern crate serde_json;

#[cfg(test)]
use proptest::prelude::*;
#[cfg(test)]
use proptest_derive::Arbitrary;

use crate::tfhe::schemas::{
    from_poly_list, from_u64, GLWE_Params, LWE_CT_Params, LWE_Params, TFHESchema,
    TFHE_test_medium_u64, TFHE_test_small_u64,
};

#[derive(Debug, PartialEq)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct GGSWCiphertext<S: TFHESchema, P: LWE_CT_Params<S>>(P::ContainerType);

impl<S: TFHESchema, P: LWE_CT_Params<S>> GGSWCiphertext<S, P> {
    // fn new(data: Box<[u64; Polynomialsize*Masksize]>) -> Self
    // where
    //   [(); Masksize+1]: Sized
    // {
    //   GLWECiphertext(data)
    // }

    pub fn from_polynomial_list(data: P::ContainerType) -> Self {
        GGSWCiphertext(data)
    }

    pub fn get_poly_by_index(&self, ind: usize) -> Polynomial<{ P::POLINOMIAL_SIZE }> {
        let mut v = Polynomial::<{ P::POLINOMIAL_SIZE }>::new_zero();
        for i in 0..P::POLINOMIAL_SIZE {
            v[i] = from_u64::to(self.0[ind * P::POLINOMIAL_SIZE + i]);
        }
        v
    }
}

impl<S: TFHESchema, P: LWE_CT_Params<S>> Display for GGSWCiphertext<S, P> {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(
            formatter,
            "{}",
            serde_json::to_string(&self.0).unwrap() // self.0
        )
        .unwrap();
        Ok(())
    }
}

impl<S: TFHESchema, P: LWE_CT_Params<S>> FromStr for GGSWCiphertext<S, P> {
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
    let ct: GGSWCiphertext<TFHE_test_small_u64, LWE_Params<TFHE_test_small_u64>> =
        GGSWCiphertext::from_polynomial_list(a);

    let serialized = ct.to_string();
    let deserialized: GGSWCiphertext<TFHE_test_small_u64, LWE_Params<TFHE_test_small_u64>> =
        FromStr::from_str(&serialized).unwrap();
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

// ops
impl<S: TFHESchema, P: LWE_CT_Params<S>> ops::Mul<&GLWECiphertext<S, P>> for &GGSWCiphertext<S, P>
where
    [(); P::POLINOMIAL_SIZE]: Sized,
    [(); S::GLEV_B]: Sized,
    [(); S::GLEV_L]: Sized,
    [(); S::GLWE_Q]: Sized,
{
    type Output = GLWECiphertext<S, P>;

    fn mul(self, rhs: &GLWECiphertext<S, P>) -> GLWECiphertext<S, P> {
        // println!("mul_ext: 1");

        let mut dec: Vec<Polynomial<{ P::POLINOMIAL_SIZE }>> = Vec::with_capacity(S::GLEV_L);
        for _ in 0..S::GLEV_L {
            dec.push(Polynomial::<{ P::POLINOMIAL_SIZE }>::new_zero())
        }
        
        let mut acc: Vec<Polynomial<{ P::POLINOMIAL_SIZE }>> = Vec::with_capacity(P::MASK_SIZE + 1);
        for _ in 0..=P::MASK_SIZE {
            acc.push(Polynomial::<{ P::POLINOMIAL_SIZE }>::new_zero())
        }
        // let mut acc: GLWECiphertext<S,P> = GLWECiphertext::from_polynomial_list(from_poly_list::from(zero_data));


        for glev_number in 0..=P::MASK_SIZE {

            decompose_polynomial_assign::<
                { S::GLWE_Q },
                { S::GLEV_L },
                { S::GLEV_B },
                { P::POLINOMIAL_SIZE },
            >(rhs.get_poly_by_index(glev_number), &mut dec);
            // println!("mul_ext: glev_number: {glev_number}, poly: {:?}, dec: {:?}", rhs.get_poly_by_index(glev_number), dec);
            let offset_glev = glev_number * (S::GLEV_L * (P::MASK_SIZE + 1));

            for glwe_number in 0..S::GLEV_L {
                let offset_glwe = glwe_number * (P::MASK_SIZE + 1);

                for poly_number in 0..=P::MASK_SIZE {
                    // println!("mul_ext: 3, get_poly_by_index offset_glev: {}, offset_glwe: {}, poly_number: {}, self[]: {:?}, dec[]: {:?}: ", offset_glev, offset_glwe, poly_number, &self.get_poly_by_index(offset_glev+offset_glwe+poly_number), &dec[glwe_number]);
                   acc[poly_number] += &(&dec[glwe_number] * &self.get_poly_by_index(offset_glev + offset_glwe + poly_number));
                }
            }
        }

        GLWECiphertext::from_polynomial_list(from_poly_list::from(acc))
    }
}



#[cfg(test)]
proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]
    #[test]
    fn pt_ggsw_mul_external_callable(a in any::<[u64; GLWE_Params::<TFHE_test_small_u64>::POLINOMIAL_SIZE
        *(GLWE_Params::<TFHE_test_small_u64>::MASK_SIZE+1)
        *(GLWE_Params::<TFHE_test_small_u64>::MASK_SIZE+1)
        *(TFHE_test_small_u64::GLEV_L)]>()
    .prop_map(|v| GGSWCiphertext::<TFHE_test_small_u64, GLWE_Params<TFHE_test_small_u64>>::from_polynomial_list(v.to_vec())),
        b in any::<[u64; GLWE_Params::<TFHE_test_small_u64>::POLINOMIAL_SIZE*(GLWE_Params::<TFHE_test_small_u64>::MASK_SIZE+1)]>()
        .prop_map(|v| GLWECiphertext::<TFHE_test_small_u64, GLWE_Params<TFHE_test_small_u64>>::from_polynomial_list(v.to_vec()))) {

        let _ = &a * &b;

    }
}
