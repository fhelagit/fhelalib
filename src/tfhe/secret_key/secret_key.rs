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
use crate::tfhe::schemas::{from_poly_list, from_u64, GLWE_Params, LWE_CT_Params, LWE_Params, TFHESchema, TFHE_test_small_u64};
// use std::ops::{Index};

#[cfg(test)]
use proptest::prelude::*;

#[derive(Debug, PartialEq)]
pub struct GLWE_secret_key<S: TFHESchema, P: LWE_CT_Params<S>>(P::SecretKeyContainerType);

pub struct GLWE_secret_key2<S: TFHESchema, P: LWE_CT_Params<S>>(P::SecretKeyContainerType);

impl<S: TFHESchema, P: LWE_CT_Params<S>> GLWE_secret_key<S, P> 
where [(); P::POLINOMIAL_SIZE]:Sized {
    pub fn new_random() -> Self {
        let mut d: Vec<Polynomial<{P::POLINOMIAL_SIZE}>> = Vec::with_capacity(S::LWE_K);
        for _ in 0..S::LWE_K {
            d.push(Polynomial::<{P::POLINOMIAL_SIZE}>::new([rnd_u64_uniform_binary(); P::POLINOMIAL_SIZE].to_vec()));
        }
        GLWE_secret_key::from_scalar_vector(from_poly_list::from(d))
    }

    // #[cfg(test)]
    pub fn from_scalar_vector(data: P::SecretKeyContainerType) -> Self {
        GLWE_secret_key(data)
    }


    pub fn get_poly_by_index(&self, ind: usize) -> Polynomial<{P::POLINOMIAL_SIZE}>{

        let mut v: Vec<u64> = Vec::with_capacity(P::POLINOMIAL_SIZE);
        for i in 0..P::POLINOMIAL_SIZE{
            v.push(from_u64::to(self.0[ind*P::POLINOMIAL_SIZE+i]));
        }
        Polynomial::<{P::POLINOMIAL_SIZE}>::new(v)

    }


    fn encript(&self, message: &Polynomial<{P::POLINOMIAL_SIZE}>) -> GLWECiphertext<S, P> {

        // создать полином шума
        let mut e: Vec<u64> = Vec::with_capacity(P::POLINOMIAL_SIZE); //[rnd_u64_gausean() ; P::POLINOMIAL_SIZE].to_vec(); 
        for _ in 0..P::POLINOMIAL_SIZE {
            e.push(rnd_u64_gausean());
        }

        let err = Polynomial::<{P::POLINOMIAL_SIZE}>::new(dbg!(e));
       // dbg!(self.0);
        // создать полиномы ашки
        let mut a_s: Vec<Polynomial<{P::POLINOMIAL_SIZE}>> = Vec::with_capacity(P::MASK_SIZE);
        for _ in 0..P::MASK_SIZE { 
            let mut a_i: Vec<u64> = Vec::with_capacity(P::POLINOMIAL_SIZE); 
            for _ in 0..P::POLINOMIAL_SIZE {
                a_i.push(rnd_u64_uniform());
            }

            a_s.push(Polynomial::new(a_i));
        }
        dbg!(&a_s);
        // посчитать мультисумму
        let mut multysum = Polynomial::<{P::POLINOMIAL_SIZE}>::new([0 ; P::POLINOMIAL_SIZE].to_vec());
        for i in 0..P::MASK_SIZE {
            multysum = dbg!(&multysum) + dbg!(&(dbg!(&a_s[i]) * dbg!(&(self.get_poly_by_index(i)))));
        }

        println!("println1");
        // cоздать сдвинутое сообщение
        let delta = (S::GLWE_Q - S::GLEV_B) as u32;
        let shifted_message = Polynomial::new(message.into_iter().map(|v| v.wrapping_shl(delta)).collect());

        println!("println2");
        // сложить все вместе

        let b = &(dbg!(&multysum) + dbg!(&shifted_message)) + dbg!(&err);
        a_s.push(dbg!(b));

        GLWECiphertext::<S, P>::from_polynomial_list(from_poly_list::from(a_s))
    }

    fn decript(&self, ct: &GLWECiphertext<S, P>) -> Polynomial<{P::POLINOMIAL_SIZE}> {

        println!("decript 1");

        // // посчитать мультисумму
        let mut multysum: Polynomial<{P::POLINOMIAL_SIZE}> = Polynomial::<{P::POLINOMIAL_SIZE}>::new([0 ; P::POLINOMIAL_SIZE].to_vec());
        for i in 0..P::MASK_SIZE {
            multysum = &multysum + &(&ct.get_poly_by_index(i) * &(self.get_poly_by_index(i)));
        }

        println!("decript 2");
        let shifted_message = dbg!(&ct.get_poly_by_index(P::MASK_SIZE)) - dbg!(&multysum);
        println!("decript 3");
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

// impl<S: TFHESchema, P: LWE_CT_Params<S>> Index<usize> for GLWE_secret_key<S, P> {
//     type Output = Polynomial<{P::POLINOMIAL_SIZE}>;

//     fn index(self, i: usize) -> Self::Output {
//         let a = self.0[i];
//         &Polynomial::<{P::POLINOMIAL_SIZE}>::new([from_u64::to(self.0[i])].to_vec())
//     }
// }


#[cfg(test)]
proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]
    #[test]
    fn pt_encript_invertable(m in any::<[u8; GLWE_Params::<TFHE_test_small_u64>::POLINOMIAL_SIZE]>().prop_map(|v| Polynomial::<{GLWE_Params::<TFHE_test_small_u64>::POLINOMIAL_SIZE}>::new(v.iter().map(|vv| *vv as u64).collect()))) {

       let sk: GLWE_secret_key<TFHE_test_small_u64, GLWE_Params<TFHE_test_small_u64>> = dbg!(GLWE_secret_key::new_random());
       let encripted: GLWECiphertext<TFHE_test_small_u64, GLWE_Params<TFHE_test_small_u64>> = sk.encript(dbg!(&m));
       let decripted = sk.decript(dbg!(&encripted));

       prop_assert_eq!(dbg!(decripted), dbg!(m));
      // assert_eq!(1,2);

 
    }
}

 #[cfg(test)]
proptest! {
    #![proptest_config(ProptestConfig::with_cases(10))]
    #[test]
    fn pt_secretkey_creatable(_ in 1..2u64) {

        let _: GLWE_secret_key<TFHE_test_small_u64, GLWE_Params<TFHE_test_small_u64>> = GLWE_secret_key::new_random();

    }
}

#[cfg(test)]
proptest! {
    #![proptest_config(ProptestConfig::with_cases(10))]
    #[test]
    fn pt_secretkey_creatable_from_polynomial_list(d in any::<[u64; 586]>().prop_map(|v| v.to_vec())) {

        let _: GLWE_secret_key<TFHE_test_small_u64, LWE_Params<TFHE_test_small_u64>> = GLWE_secret_key::from_scalar_vector(d);

    }
}


#[cfg(test)]
proptest! {
    #![proptest_config(ProptestConfig::with_cases(1000))]
    #[test]
    fn pt_glwe_ct_add(a in any::<[u8; GLWE_Params::<TFHE_test_small_u64>::POLINOMIAL_SIZE]>().prop_map(|v| Polynomial::<{GLWE_Params::<TFHE_test_small_u64>::POLINOMIAL_SIZE}>::new(v.iter().map(|vv| (*vv >> 4) as u64).collect())), 
                      b in any::<[u8; GLWE_Params::<TFHE_test_small_u64>::POLINOMIAL_SIZE]>().prop_map(|v| Polynomial::<{GLWE_Params::<TFHE_test_small_u64>::POLINOMIAL_SIZE}>::new(v.iter().map(|vv| (*vv >> 4) as u64).collect())))  {

        let sk: GLWE_secret_key<TFHE_test_small_u64, GLWE_Params<TFHE_test_small_u64>> = dbg!(GLWE_secret_key::new_random());

        let encripted_a: GLWECiphertext<TFHE_test_small_u64, GLWE_Params<TFHE_test_small_u64>> = sk.encript(dbg!(&a));
        let encripted_b: GLWECiphertext<TFHE_test_small_u64, GLWE_Params<TFHE_test_small_u64>> = sk.encript(dbg!(&b));
        println!("pt_glwe_ct_add 1");
        let sum = dbg!(&encripted_a) + &encripted_b;
        println!("pt_glwe_ct_add 2");

        //здесь
        let decripted_sum = sk.decript(dbg!(&sum));
        println!("pt_glwe_ct_add 3");
        let expected_sum = dbg!(&a) + dbg!(&b);
        println!("pt_glwe_ct_add 4");

        prop_assert_eq!(decripted_sum, expected_sum);

    }
}


#[cfg(test)]
proptest! {
    #![proptest_config(ProptestConfig::with_cases(1000))]
    #[test]
    fn pt_glwe_ct_sub(a in any::<[u8; GLWE_Params::<TFHE_test_small_u64>::POLINOMIAL_SIZE]>().prop_map(|v| Polynomial::<{GLWE_Params::<TFHE_test_small_u64>::POLINOMIAL_SIZE}>::new(v.iter().map(|vv| (*vv >> 4) as u64).collect())), 
                      b in any::<[u8; GLWE_Params::<TFHE_test_small_u64>::POLINOMIAL_SIZE]>().prop_map(|v| Polynomial::<{GLWE_Params::<TFHE_test_small_u64>::POLINOMIAL_SIZE}>::new(v.iter().map(|vv| (*vv >> 4) as u64).collect())))  {

        let sk: GLWE_secret_key<TFHE_test_small_u64, GLWE_Params<TFHE_test_small_u64>> = dbg!(GLWE_secret_key::new_random());

        let encripted_a: GLWECiphertext<TFHE_test_small_u64, GLWE_Params<TFHE_test_small_u64>> = sk.encript(dbg!(&a));
        let encripted_b: GLWECiphertext<TFHE_test_small_u64, GLWE_Params<TFHE_test_small_u64>> = sk.encript(dbg!(&b));
        println!("pt_glwe_ct_sub 1");
        let diff = dbg!(&encripted_a) - &encripted_b;
        println!("pt_glwe_ct_sub 2");

        //здесь
        let decripted_diff = sk.decript(dbg!(&diff));
        println!("pt_glwe_ct_sub 3");
        let expected_diff = dbg!(&a) - dbg!(&b);
        println!("pt_glwe_ct_sub 4");

        prop_assert_eq!(decripted_diff, expected_diff);

    }
}

