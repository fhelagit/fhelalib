#![allow(non_camel_case_types)]

use crate::math::polynomial::polynomial::Polynomial;
use crate::random::random::rnd_u64_gausean;
use crate::tfhe::ggsw::ggsw::GGSWCiphertext;
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


    fn encrypt(&self, message: &Polynomial<{P::POLINOMIAL_SIZE}>) -> GLWECiphertext<S, P> {

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

    fn decrypt(&self, ct: &GLWECiphertext<S, P>) -> Polynomial<{P::POLINOMIAL_SIZE}> {

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

    fn encript_ggsw(&self, message: &Polynomial<{P::POLINOMIAL_SIZE}>) -> GGSWCiphertext<S, P> {

        let mut ct_data: Vec<Polynomial<{P::POLINOMIAL_SIZE}>> = Vec::with_capacity((P::MASK_SIZE+1) * S::GLEV_L*(P::MASK_SIZE+1));

       // получить все варианты сообщения
        for i in 0..P::MASK_SIZE {
           self.encrypt_glev(&(message * &(&Polynomial::new_zero() - &self.get_poly_by_index(i))), &mut ct_data);
        }
        self.encrypt_glev(message, &mut ct_data).unwrap();

        GGSWCiphertext::from_polynomial_list(from_poly_list::from(ct_data))
    }

    pub(self) fn encrypt_glev(&self, message: &Polynomial<{P::POLINOMIAL_SIZE}>, acc: &mut Vec<Polynomial<{P::POLINOMIAL_SIZE}>> ) -> Result<(), ()> {
        for i in 1..(S::GLEV_L + 1) {
            let ct = self.encrypt(&Polynomial::new(message.into_iter().map(|v| v << S::GLWE_Q - S::GLEV_B * i).collect()));
            for i in 0..P::MASK_SIZE+1 {
                acc.push(ct.get_poly_by_index(i));
            }
        }
        Ok(())
    }

}

#[cfg(test)]
proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]
    #[test]
    fn pt_encrypt_invertable(m in any::<[u8; GLWE_Params::<TFHE_test_small_u64>::POLINOMIAL_SIZE]>().prop_map(|v| Polynomial::<{GLWE_Params::<TFHE_test_small_u64>::POLINOMIAL_SIZE}>::new(v.iter().map(|vv| *vv as u64).collect()))) {

       let sk: GLWE_secret_key<TFHE_test_small_u64, GLWE_Params<TFHE_test_small_u64>> = dbg!(GLWE_secret_key::new_random());
       let encripted: GLWECiphertext<TFHE_test_small_u64, GLWE_Params<TFHE_test_small_u64>> = sk.encrypt(dbg!(&m));
       let decripted = sk.decrypt(dbg!(&encripted));

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

        let encripted_a: GLWECiphertext<TFHE_test_small_u64, GLWE_Params<TFHE_test_small_u64>> = sk.encrypt(dbg!(&a));
        let encripted_b: GLWECiphertext<TFHE_test_small_u64, GLWE_Params<TFHE_test_small_u64>> = sk.encrypt(dbg!(&b));
        println!("pt_glwe_ct_add 1");
        let sum = dbg!(&encripted_a) + &encripted_b;
        println!("pt_glwe_ct_add 2");

        //здесь
        let decripted_sum = sk.decrypt(dbg!(&sum));
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

        let encripted_a: GLWECiphertext<TFHE_test_small_u64, GLWE_Params<TFHE_test_small_u64>> = sk.encrypt(dbg!(&a));
        let encripted_b: GLWECiphertext<TFHE_test_small_u64, GLWE_Params<TFHE_test_small_u64>> = sk.encrypt(dbg!(&b));
        println!("pt_glwe_ct_sub 1");
        let diff = dbg!(&encripted_a) - &encripted_b;
        println!("pt_glwe_ct_sub 2");

        //здесь
        let decripted_diff = sk.decrypt(dbg!(&diff));
        println!("pt_glwe_ct_sub 3");
        let expected_diff = dbg!(&a) - dbg!(&b);
        println!("pt_glwe_ct_sub 4");

        prop_assert_eq!(decripted_diff, expected_diff);

    }
}

