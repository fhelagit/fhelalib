#![allow(non_camel_case_types)]

use crate::math::polynomial::polynomial::Polynomial;
use crate::{
    // math::polynomial::polynomial::Polynomial, 
    random::random::rnd_u64_uniform_binary,
    // tfhe::glwe::GLWECiphertext,
};
use crate::tfhe::schemas::{from_Poly_list, TFHESchema, TFHE_test_medium_u64};

#[cfg(test)]
use proptest::prelude::*;

#[derive(Debug, PartialEq)]
pub struct GLWE_secret_key<S: TFHESchema>(S::SecretKeyContainerType);

impl<S: TFHESchema> GLWE_secret_key<S> {
    pub fn new_random() -> Self {
        let mut d: Vec<Polynomial<1>> = Vec::with_capacity(S::GLWE_K);
        for i in 0..S::GLWE_K {
            d.push(Polynomial::new(Box::new([rnd_u64_uniform_binary()])));
        }
        GLWE_secret_key::from_scalar_vector(from_Poly_list::from(d))
    }

    #[cfg(test)]
    pub fn from_scalar_vector(data: S::SecretKeyContainerType) -> Self {
        GLWE_secret_key(data)
    }

    //   pub fn encript(
    //     m: &Poly<POLY_SIZE>,
    //     GLWESecretKey(ss): &GLWESecretKey<POLY_SIZE, K>,
    //     custom_delta: Option<u64>,
    // ) -> GLWECiphertext<POLY_SIZE, K> {
    //     //let err = random_uniform_binary();
    //     let err = zero_poly::<POLY_SIZE>();
    //     // dbg!(e);
    //     let delta: u64 = match custom_delta {
    //         Some(d) => d,
    //         _ => 2_u64.pow((GLWE_Q - GLEV_B) as u32),
    //     };
    //     // dbg!("enc delta: {}", delta);
    //     // dbg!(delta);
    //     let dm = m * delta  as usize;
    //     // dbg!(dm);
    //     let mut mask_vec: Vec<Poly<POLY_SIZE>> = Vec::new();

    //     for _ in 0..K {
    //         mask_vec.push( //zero_poly::<POLY_SIZE>());
    //         random_uniform_glwe_q();
    //     }

    //     let mask: [Poly<POLY_SIZE>; K] = mask_vec.try_into().unwrap();

    //     let mask2 = mask.clone();
    //     //     .iter()
    //     //     .map(|_| (random_uniform_glwe_q()))
    //     //     .collect::<Vec<Poly<POLY_SIZE>>>()
    //     //     .try_into()
    //     //     .unwrap();
    //     // dbg!(mask);
    //     let ListOfPoly(mul_mask_skey) = &ListOfPoly(mask2) * &ListOfPoly(ss.clone());
    //     let mul_sum_mask_skey = mul_mask_skey
    //         .iter()
    //         .fold(Poly::<POLY_SIZE>::new([ModNumber(0); POLY_SIZE].to_vec()), |acc, e| {
    //             acc + e
    //         });

    //     let body = &(mul_sum_mask_skey + &dm) + &err;
    //     GLWECiphertext { mask, body }
    // }
    // fn encript(self, message: Polynomial<Polynomialsize>) -> GLWECiphertext {
    //   let err = zero_poly::<POLY_SIZE>();
    //   let delta: u64 = 2_u64.pow((GLWE_Q - GLEV_B) as u32);
    //   GLWECiphertext::from_polynomial_list(data)
    // }
}

 // fn pt_secretkey_creatable(ct in any::<[u64; 1024*2]>().prop_map(|v| GLWECiphertext::<GLWE_Params<TFHE_test_medium_u64>>::from_polynomial_list(v.to_vec())))
#[cfg(test)]
proptest! {
    #![proptest_config(ProptestConfig::with_cases(10))]
    #[test]
    fn pt_secretkey_creatable(_ in 1..2u64) {

        let sk: GLWE_secret_key<TFHE_test_medium_u64> = GLWE_secret_key::new_random();

    }
}

#[cfg(test)]
proptest! {
    #![proptest_config(ProptestConfig::with_cases(10))]
    #[test]
    fn pt_secretkey_creatable_from_polynomial_list(d in any::<[u64; 586]>().prop_map(|v| v.to_vec())) {

        let sk: GLWE_secret_key<TFHE_test_medium_u64> = GLWE_secret_key::from_scalar_vector(d);

    }
}

