#![allow(non_camel_case_types)]

use crate::math::polynomial::polynomial::Polynomial;
use crate::random::random::rnd_u64_gausean;
use crate::tfhe::glwe::GLWECiphertext;
use crate::{
    // math::polynomial::polynomial::Polynomial, 
    random::random::rnd_u64_uniform_binary,
    random::random::rnd_u64_uniform,
    // tfhe::glwe::GLWECiphertext,
};
use crate::tfhe::schemas::{from_poly_list, from_u64, LWE_Params, TFHESchema, TFHE_test_medium_u64, TFHE_test_small_u64, LWE_CT_Params};
use std::ops::{Index};

#[cfg(test)]
use proptest::prelude::*;

#[derive(Debug, PartialEq)]
pub struct GLWE_secret_key<S: TFHESchema, P: LWE_CT_Params<S>>(P::SecretKeyContainerType);

impl<S: TFHESchema, P: LWE_CT_Params<S>> GLWE_secret_key<S, P> {
    pub fn new_random() -> Self {
        let mut d: Vec<Polynomial<1>> = Vec::with_capacity(S::LWE_K);
        for _ in 0..S::LWE_K {
            d.push(Polynomial::new([rnd_u64_uniform_binary()].to_vec()));
        }
        GLWE_secret_key::from_scalar_vector(from_poly_list::from(d))
    }

    #[cfg(test)]
    pub fn from_scalar_vector(data: P::SecretKeyContainerType) -> Self {
        GLWE_secret_key(data)
    }


    pub fn get_poly_by_index(&self, ind: usize) -> Polynomial<1>{
        Polynomial::<1>::new([from_u64::to(self.0[ind])].to_vec())
    }


    fn encript(&self, message: &Polynomial<1>) -> GLWECiphertext<S, P> {

        // создать полином шума
        let e = [rnd_u64_gausean() ; 1].to_vec(); 
        let err = Polynomial::<1>::new(dbg!(e));
       // dbg!(self.0);
        // создать полиномы ашки
        let mut a_s: Vec<Polynomial<1>> = Vec::with_capacity(S::LWE_K);
        for _ in 0..S::LWE_K {
            a_s.push(Polynomial::new([rnd_u64_uniform()].to_vec()));
        }
            dbg!(&a_s);
        // посчитать мультисумму
        let mut multysum = Polynomial::<1>::new([0 ; 1].to_vec());
        for i in 0..S::LWE_K {
            multysum = &multysum + &(&a_s[i] * &(self.get_poly_by_index(i)));
        }


        // cоздать сдвинутое сообщение
        let delta = (S::GLWE_Q - S::GLEV_B) as u32;
        let shifted_message = Polynomial::new(message.into_iter().map(|v| v.wrapping_shl(delta)).collect());

        // сложить все вместе

        let b = &(dbg!(&multysum) + dbg!(&shifted_message)) + dbg!(&err);
        a_s.push(dbg!(b));

        GLWECiphertext::<S, P>::from_polynomial_list(from_poly_list::from(a_s))
    }

    fn decript(&self, ct: &GLWECiphertext<S, P>) -> Polynomial<1> {



        // // посчитать мультисумму
        let mut multysum = Polynomial::<1>::new([0 ; 1].to_vec());
        for i in 0..S::LWE_K {
            multysum = &multysum + &(&ct.get_poly_by_index(i) * &(self.get_poly_by_index(i)));
        }

        let shifted_message = dbg!(&ct.get_poly_by_index(S::LWE_K)) - dbg!(&multysum);
        let delta = (S::GLWE_Q - S::GLEV_B) as u32;
        let message = Polynomial::new(dbg!(shifted_message).into_iter().map(|v| v.wrapping_shr(delta)).collect());

        // // cоздать сдвинутое сообщение
        // let delta: u64 = 2_u64.pow((S::GLWE_Q - S::GLEV_B) as u32);
        // let shifted_message = Polynomial::new(message.into_iter().map(|v| v << delta).collect());

        // // сложить все вместе

        // let b = &(&multysum + &shifted_message) + &err;
        // a_s.push(b);

        // GLWECiphertext::<LWE_Params<S>>::from_polynomial_list(from_poly_list::from(a_s))

        message
    }
}

// impl<S: TFHESchema> Index<usize> for GLWE_secret_key<S> {
//     type Output = Polynomial<1>;

//     fn index(self, i: usize) -> Self::Output {
//         let a = self.0[i];
//         &Polynomial::<1>::new([from_u64::to(self.0[i])].to_vec())
//     }
// }

 // fn pt_secretkey_creatable(ct in any::<[u64; 1024*2]>().prop_map(|v| GLWECiphertext::<GLWE_Params<TFHE_test_medium_u64>>::from_polynomial_list(v.to_vec())))

// #[cfg(test)]
// proptest! {
//     #![proptest_config(ProptestConfig::with_cases(100000))]
//     #[test]
//     fn pt_encript_invertable(m in any::<u8>().prop_map(|v| Polynomial::<1>::new([v as u64; 1].to_vec()))) {

//        let sk: GLWE_secret_key<TFHE_test_small_u64> = GLWE_secret_key::new_random();
//        let encripted: GLWECiphertext<TFHE_test_small_u64> = sk.encript(dbg!(&m));
//        let decripted = sk.decript(dbg!(&encripted));

//        prop_assert_eq!(dbg!(decripted), dbg!(m));
//       // assert_eq!(1,2);

 
//     }
// }

//  #[cfg(test)]
// proptest! {
//     #![proptest_config(ProptestConfig::with_cases(10))]
//     #[test]
//     fn pt_secretkey_creatable(_ in 1..2u64) {

//         let _: GLWE_secret_key<TFHE_test_medium_u64> = GLWE_secret_key::new_random();

//     }
// }

// #[cfg(test)]
// proptest! {
//     #![proptest_config(ProptestConfig::with_cases(10))]
//     #[test]
//     fn pt_secretkey_creatable_from_polynomial_list(d in any::<[u64; 586]>().prop_map(|v| v.to_vec())) {

//         let _: GLWE_secret_key<TFHE_test_medium_u64> = GLWE_secret_key::from_scalar_vector(d);

//     }
// }

