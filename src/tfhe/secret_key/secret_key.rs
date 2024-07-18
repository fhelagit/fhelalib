#![allow(non_camel_case_types)]

use crate::math::polynomial::polynomial::Polynomial;
use crate::random::random::{rnd_u64_gausean, rnd_u64_uniform_bounded};
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
            e.push(0);//rnd_u64_gausean());
        }
        println!("encrypt.noise: {:?}", e);

        let err = Polynomial::<{P::POLINOMIAL_SIZE}>::new(e);
       // dbg!(self.0);
        // создать полиномы ашки
        let mut a_s: Vec<Polynomial<{P::POLINOMIAL_SIZE}>> = Vec::with_capacity(P::MASK_SIZE);
        for _ in 0..P::MASK_SIZE { 
            let mut a_i: Vec<u64> = Vec::with_capacity(P::POLINOMIAL_SIZE); 
            for _ in 0..P::POLINOMIAL_SIZE {
                a_i.push(rnd_u64_uniform_bounded(1<<56));//rnd_u64_uniform());
            }

            a_s.push(Polynomial::new(a_i));
        }
        println!("encrypt.a_s: {:?}", a_s);
        // посчитать мультисумму
        let mut multysum = Polynomial::<{P::POLINOMIAL_SIZE}>::new([0 ; P::POLINOMIAL_SIZE].to_vec());
        for i in 0..P::MASK_SIZE {
            multysum = dbg!(&multysum) + dbg!(&(dbg!(&a_s[i]) * dbg!(&(self.get_poly_by_index(i)))));
        }

        println!("println1");
        // cоздать сдвинутое сообщение
        let _delta = (S::GLWE_Q - S::GLEV_B) as u32;
        let shifted_message = message;// Polynomial::new(message.into_iter().map(|v| v.wrapping_shl(delta)).collect());

        println!("println2");
        // сложить все вместе

        let body = &(dbg!(&multysum) + dbg!(&shifted_message)) + dbg!(&err);
        a_s.push(dbg!(body));

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
        let _delta = (S::GLWE_Q - S::GLEV_B) as u32;
        let message = shifted_message;//Polynomial::new(dbg!(shifted_message).into_iter().map(|v| v.wrapping_shr(delta)).collect());

        // // cоздать сдвинутое сообщение
        // let delta: u64 = 2_u64.pow((S::GLWE_Q - S::GLEV_B) as u32);
        // let shifted_message = Polynomial::new(message.into_iter().map(|v| v << delta).collect());

        // // сложить все вместе

        // let b = &(&multysum + &shifted_message) + &err;
        // a_s.push(b);

        // GLWECiphertext::<LWE_Params<S>>::from_polynomial_list(from_poly_list::from(a_s))

        message
    }

    pub fn encrypt_ggsw(&self, message: &Polynomial<{P::POLINOMIAL_SIZE}>) -> GGSWCiphertext<S, P> {

        let mut ct_data: Vec<Polynomial<{P::POLINOMIAL_SIZE}>> = Vec::with_capacity((P::MASK_SIZE+1) * S::GLEV_L*(P::MASK_SIZE+1));
        println!("encrypt_ggsw.message: {}", message);

       // получить все варианты сообщения
        for i in 0..P::MASK_SIZE {
            let message_ = &(message * &(&Polynomial::new_zero() - &self.get_poly_by_index(i)));
            println!("encrypt_ggsw.message_: {}", message_);
            self.encrypt_glev(message_, &mut ct_data).unwrap();
            println!("encrypt_ggsw.ct_data: {:#?}", ct_data);
        }
        self.encrypt_glev(message, &mut ct_data).unwrap();

        println!("ggsw.container.len(): {}", ct_data.len());

        GGSWCiphertext::from_polynomial_list(from_poly_list::from(ct_data))
    }

    fn encrypt_glev(&self, message: &Polynomial<{P::POLINOMIAL_SIZE}>, acc: &mut Vec<Polynomial<{P::POLINOMIAL_SIZE}>> ) -> Result<(), ()> {
        println!("encrypt_glev.message: {}", message);
        for i in 1..=S::GLEV_L {
            let message_ = &Polynomial::new(message.into_iter().map(|v| v << (S::GLWE_Q - S::GLEV_B * i)).collect());
            println!("encrypt_glev.message_: {}", message_);
            let ct = self.encrypt(message_);
            println!("encrypt_glev.ct: {}", ct);
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


#[cfg(test)]
proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]
    #[test]
    fn pt_encrypt_ggsw_callable(m in any::<[u8; GLWE_Params::<TFHE_test_small_u64>::POLINOMIAL_SIZE]>().prop_map(|v| Polynomial::<{GLWE_Params::<TFHE_test_small_u64>::POLINOMIAL_SIZE}>::new(v.iter().map(|vv| *vv as u64).collect()))) {

       let sk: GLWE_secret_key<TFHE_test_small_u64, GLWE_Params<TFHE_test_small_u64>> = dbg!(GLWE_secret_key::new_random());
       let _encripted: GGSWCiphertext<TFHE_test_small_u64, GLWE_Params<TFHE_test_small_u64>> = sk.encrypt_ggsw(dbg!(&m));
    //    let decripted = sk.decrypt(dbg!(&encripted));

    //    prop_assert_eq!(dbg!(decripted), dbg!(m));
      // assert_eq!(1,2);

 
    }
}

#[cfg(test)]
proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]
    #[test]
    fn pt_ggsw_mul_external_expected(a in any::<[u8; GLWE_Params::<TFHE_test_small_u64>::POLINOMIAL_SIZE]>().prop_map(|v| Polynomial::<{GLWE_Params::<TFHE_test_small_u64>::POLINOMIAL_SIZE}>::new(v.iter().map(|vv| (*vv >> 4) as u64).collect())),
                            b in any::<[u8; GLWE_Params::<TFHE_test_small_u64>::POLINOMIAL_SIZE]>().prop_map(|v| Polynomial::<{GLWE_Params::<TFHE_test_small_u64>::POLINOMIAL_SIZE}>::new(v.iter().map(|vv| ((*vv >> 4) as u64) <<56).collect()))) {

        let sk: GLWE_secret_key<TFHE_test_small_u64, GLWE_Params<TFHE_test_small_u64>> = dbg!(GLWE_secret_key::new_random());

        let encripted_a: GGSWCiphertext<TFHE_test_small_u64, GLWE_Params<TFHE_test_small_u64>> = sk.encrypt_ggsw(dbg!(&a));
        let encripted_b: GLWECiphertext<TFHE_test_small_u64, GLWE_Params<TFHE_test_small_u64>> = sk.encrypt(dbg!(&b));
        let decripted_b = sk.decrypt(dbg!(&encripted_b));
     

        let expected_product =  &a * &b;

        let product = dbg!(&encripted_a) * &encripted_b;
    

        let decripted_product = sk.decrypt(dbg!(&product));

        prop_assert_eq!(dbg!(decripted_product.into_iter().map(|v| v>>56).collect::<Vec<u64>>()), dbg!(expected_product.into_iter().map(|v| v>>56).collect::<Vec<u64>>()));



    }
}

#[cfg(test)]
proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]
    #[test]
    fn pt_ggsw_mul_external_neutral_rhs(a in any::<[u8; GLWE_Params::<TFHE_test_small_u64>::POLINOMIAL_SIZE]>().prop_map(|v| Polynomial::<{GLWE_Params::<TFHE_test_small_u64>::POLINOMIAL_SIZE}>::new(v.iter().map(|vv| (*vv >> 4) as u64).collect()))) {

        let sk: GLWE_secret_key<TFHE_test_small_u64, GLWE_Params<TFHE_test_small_u64>> = dbg!(GLWE_secret_key::new_random());
        let mut b_: Vec<u64> = Vec::new();
        for _ in 0..GLWE_Params::<TFHE_test_small_u64>::POLINOMIAL_SIZE {
            b_.push(0);
        }
        b_[0]=1 << 56;
        let b = Polynomial::<{GLWE_Params::<TFHE_test_small_u64>::POLINOMIAL_SIZE}>::new(b_);

        let encrypted_a: GGSWCiphertext<TFHE_test_small_u64, GLWE_Params<TFHE_test_small_u64>> = sk.encrypt_ggsw(dbg!(&a));
        let encrypted_b: GLWECiphertext<TFHE_test_small_u64, GLWE_Params<TFHE_test_small_u64>> = sk.encrypt(dbg!(&b));
        let decrypted_b = sk.decrypt(dbg!(&encrypted_b));
     

        let expected_product =  &a * &b;

        let product = dbg!(&encrypted_a) * &encrypted_b;
    

        let decrypted_product = sk.decrypt(dbg!(&product));

        prop_assert_eq!(dbg!(decrypted_product.into_iter().map(|v| v>>56).collect::<Vec<u64>>()), dbg!(expected_product.into_iter().map(|v| v>>56).collect::<Vec<u64>>()));



    }
}

#[cfg(test)]
proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]
    #[test]
    fn pt_ggsw_mul_external_absorbing_rhs(a in any::<[u8; GLWE_Params::<TFHE_test_small_u64>::POLINOMIAL_SIZE]>().prop_map(|v| Polynomial::<{GLWE_Params::<TFHE_test_small_u64>::POLINOMIAL_SIZE}>::new(v.iter().map(|vv| (*vv >> 4) as u64).collect()))) {

        let sk: GLWE_secret_key<TFHE_test_small_u64, GLWE_Params<TFHE_test_small_u64>> = dbg!(GLWE_secret_key::new_random());
        let mut b_: Vec<u64> = Vec::new();
        for _ in 0..GLWE_Params::<TFHE_test_small_u64>::POLINOMIAL_SIZE {
            b_.push(0);
        }
        let b = Polynomial::<{GLWE_Params::<TFHE_test_small_u64>::POLINOMIAL_SIZE}>::new(b_);

        let encrypted_a: GGSWCiphertext<TFHE_test_small_u64, GLWE_Params<TFHE_test_small_u64>> = sk.encrypt_ggsw(dbg!(&a));
        let encrypted_b: GLWECiphertext<TFHE_test_small_u64, GLWE_Params<TFHE_test_small_u64>> = sk.encrypt(dbg!(&b));
      //  let decrypted_b = sk.decrypt(dbg!(&encrypted_b));
     

        let expected_product =  dbg!((&a * &b).into_iter().map(|v| (v.wrapping_shr(56)) as u8).collect::<Vec<u8>>());
        let product = dbg!(&encrypted_a) * &encrypted_b;
    

        let decrypted_product = dbg!(sk.decrypt(dbg!(&product)).into_iter().map(|v| (v.wrapping_shr(56)) as u8).collect::<Vec<u8>>());
        println!("expected_product: {}", expected_product[0]);
        println!("decripted_product: {}, {}, {}", decrypted_product[0], decrypted_product[0] == 0 , decrypted_product[0] == 255);

        prop_assert_eq!(expected_product[0], decrypted_product[0]);

        println!("decripted_product: {}, {}, {}", decrypted_product[0], decrypted_product[0] == 0 , decrypted_product[0] == 255);



    }
}
