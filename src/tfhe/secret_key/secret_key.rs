#![allow(non_camel_case_types)]

use std::thread::sleep_ms;

use crate::math::polynomial::polynomial::Polynomial;
use crate::random::random::{rnd_u64_gausean, rnd_u64_uniform_bounded};
use crate::tfhe::ggsw::ggsw::GGSWCiphertext;
use crate::tfhe::glwe::GLWECiphertext;
use crate::tfhe::schemas::{
    from_poly_list, from_u64, from_u64_vector, GLWE_Params, LWE_CT_Params, LWE_Params, LWE_Params_after_extraction, TFHESchema, TFHE_test_small_u64
};
use crate::tfhe::server_key::cmux::cmux;
use crate::tfhe::server_key::extract_sample::extract_sample;
use crate::tfhe::server_key::server_key::{BootstrappingKey, KeyswitchingKey};
use crate::{
    random::random::rnd_u64_uniform,
    // tfhe::glwe::GLWECiphertext,
    // math::polynomial::polynomial::Polynomial,
    random::random::rnd_u64_uniform_binary,
};
// use std::ops::{Index};

#[cfg(test)]
use proptest::prelude::*;

#[derive(Debug, PartialEq)]
pub struct GLWE_secret_key<S: TFHESchema, P: LWE_CT_Params<S>> (P::SecretKeyContainerType);

impl<S: TFHESchema, P: LWE_CT_Params<S>> GLWE_secret_key<S, P>
where
    [(); P::POLINOMIAL_SIZE]: Sized,
{
    pub fn new_random() -> Self {
        let mut d: Vec<Polynomial<{ P::POLINOMIAL_SIZE }>> = Vec::with_capacity(S::LWE_K);
        for _ in 0..S::LWE_K {
            d.push(Polynomial::<{ P::POLINOMIAL_SIZE }>::new(
                [from_u64::to(P::random_scalar_key()); P::POLINOMIAL_SIZE].to_vec(),
            ));
        }
        GLWE_secret_key::from_scalar_vector(from_poly_list::from(d))
    }

    // #[cfg(test)]
    pub fn from_scalar_vector(data: P::SecretKeyContainerType) -> Self {
        GLWE_secret_key(data)
    }

    pub fn get_poly_by_index(&self, ind: usize) -> Polynomial<{ P::POLINOMIAL_SIZE }> {
        let mut v = Polynomial::<{ P::POLINOMIAL_SIZE }>::new_zero();
        for i in 0..P::POLINOMIAL_SIZE {
            v[i] = from_u64::to(self.0[ind * P::POLINOMIAL_SIZE + i]);
        }
        v
    }

    fn encrypt(&self, message: &Polynomial<{ P::POLINOMIAL_SIZE }>) -> GLWECiphertext<S, P> {
        // создать полином шума
        let mut err = Polynomial::<{P::POLINOMIAL_SIZE}>::new_zero(); 
        for elem_number in 0..P::POLINOMIAL_SIZE {
            err[elem_number] = from_u64::to(P::random_scalar_noise());
        }
        // println!("encrypt.noise: {:?}", err);

  
        // создать полиномы ашки
        let mut a_s: Vec<Polynomial<{ P::POLINOMIAL_SIZE }>> = Vec::with_capacity(P::MASK_SIZE);
        for _poly_number in 0..P::MASK_SIZE {
            let mut a_i = Polynomial::<{P::POLINOMIAL_SIZE}>::new_zero();
            for elem_number in 0..P::POLINOMIAL_SIZE {
                a_i[elem_number] = from_u64::to(P::random_scalar_mask());
            }

            a_s.push(a_i);
        }
        // println!("encrypt.a_s: {:?}", a_s);
        // посчитать мультисумму
        let mut multysum =
            Polynomial::<{ P::POLINOMIAL_SIZE }>::new_zero();
        for i in 0..P::MASK_SIZE {
            multysum =
                &multysum + &(&a_s[i] * &(self.get_poly_by_index(i)));
        }

        // println!("println1");
        // cоздать сдвинутое сообщение
        let _delta = (S::GLWE_Q - S::GLEV_B) as u32;
        let shifted_message = message; // Polynomial::new(message.into_iter().map(|v| v.wrapping_shl(delta)).collect());

        // println!("println2");
        // сложить все вместе

        let body = &(&multysum + &shifted_message) + &err;
        a_s.push(body);

        GLWECiphertext::<S, P>::from_polynomial_list(from_poly_list::from(a_s))
    }

    pub fn extract_key<P_new: LWE_CT_Params<S>>(&self) -> GLWE_secret_key<S, P_new> {
        assert_eq!(P_new::POLINOMIAL_SIZE, 1);
        let v = from_u64_vector::to(self.0.clone());
        let v2 = from_u64_vector::from(v);
        let new_sk: GLWE_secret_key<S, P_new> = GLWE_secret_key::<S, P_new>(
            v2
        );
        new_sk
    }

    fn decrypt(&self, ct: &GLWECiphertext<S, P>) -> Polynomial<{ P::POLINOMIAL_SIZE }> {
        // println!("decript 1");

        // // посчитать мультисумму
        let mut multysum: Polynomial<{ P::POLINOMIAL_SIZE }> =
            Polynomial::<{ P::POLINOMIAL_SIZE }>::new_zero();
        for i in 0..P::MASK_SIZE {
            multysum = &multysum + &(&ct.get_poly_by_index(i) * &(self.get_poly_by_index(i)));
        }

        // println!("decript 2");
        let shifted_message = &ct.get_poly_by_index(P::MASK_SIZE) - &multysum;
        // println!("decript 3");
        let _delta = (S::GLWE_Q - S::GLEV_B) as u32;
        let message = shifted_message; //Polynomial::new(shifted_message).into_iter().map(|v| v.wrapping_shr(delta)).collect();

        // // cоздать сдвинутое сообщение
        // let delta: u64 = 2_u64.pow((S::GLWE_Q - S::GLEV_B) as u32);
        // let shifted_message = Polynomial::new(message.into_iter().map(|v| v << delta).collect());

        // // сложить все вместе

        // let b = &(&multysum + &shifted_message) + &err;
        // a_s.push(b);

        // GLWECiphertext::<LWE_Params<S>>::from_polynomial_list(from_poly_list::from(a_s))

        message
    }

    pub fn encrypt_ggsw(
        &self,
        message: &Polynomial<{ P::POLINOMIAL_SIZE }>,
    ) -> GGSWCiphertext<S, P> {
        let mut ct_data: Vec<Polynomial<{ P::POLINOMIAL_SIZE }>> =
            Vec::with_capacity((P::MASK_SIZE + 1) * S::GLEV_L * (P::MASK_SIZE + 1));
        // println!("encrypt_ggsw.message: {}", message);

        // получить все варианты сообщения
        for elem_number in 0..P::MASK_SIZE {
            let message_ = &(message * &(&Polynomial::new_zero() - &self.get_poly_by_index(elem_number)));
            //println!("encrypt_ggsw.message_: {}", message_);
            self.encrypt_glev(message_, &mut ct_data).unwrap();
            // for i in 0..=P::MASK_SIZE {
            //     println!("encrypt_ggsw.ct_data[{i}]: {:?}", ct_data[i]);
            // }
        }
        self.encrypt_glev(message, &mut ct_data).unwrap();

        // println!("ggsw.container.len(): {}", ct_data.len());

        GGSWCiphertext::from_polynomial_list(from_poly_list::from(ct_data))
    }

    fn encrypt_glev(
        &self,
        message: &Polynomial<{ P::POLINOMIAL_SIZE }>,
        acc: &mut Vec<Polynomial<{ P::POLINOMIAL_SIZE }>>,
    ) -> Result<(), ()> {
        // println!("encrypt_glev.message: {}", message);
        for i in 1..=S::GLEV_L {
            let message_ = &Polynomial::new(
                message
                    .into_iter()
                    .map(|v| v << (S::GLWE_Q - S::GLEV_B * i))
                    .collect(),
            );
            // println!("encrypt_glev.message_: {}", message_);
            let ct = self.encrypt(message_);
            // println!("encrypt_glev.ct: {}", ct);
            for i in 0..P::MASK_SIZE + 1 {
                acc.push(ct.get_poly_by_index(i));
            }
        }
        Ok(())
    }

    pub fn create_bootstrapping_key<P_old: LWE_CT_Params<S>>(
        &self,
        old_key: &GLWE_secret_key<S, P_old>,
    ) -> BootstrappingKey<S, P_old, P>
    where
        [(); P_old::POLINOMIAL_SIZE]: Sized,
    {
        let mut ggsws: Vec<GGSWCiphertext<S, P>> = Vec::with_capacity(P_old::MASK_SIZE);
        for bit_number in 0..P_old::MASK_SIZE {

            let monom: Polynomial<{ P::POLINOMIAL_SIZE }> = Polynomial::new_monomial(old_key.get_poly_by_index(bit_number)[0], 0);
            
            ggsws.push(self.encrypt_ggsw(&monom));
        }

        BootstrappingKey::from_vec(ggsws)
    }

    pub fn create_keyswitching_key<P_old: LWE_CT_Params<S>>(
        &self,
        old_key: &GLWE_secret_key<S, P_old>
    ) -> KeyswitchingKey<S, P_old, P> 
    where
        [(); P_old::POLINOMIAL_SIZE]: Sized,
        {
        assert_eq!(P_old::POLINOMIAL_SIZE, 1);
        assert_eq!(P::POLINOMIAL_SIZE, 1);
        let mut ct_data: Vec<Polynomial<{ P::POLINOMIAL_SIZE }>> =
            Vec::with_capacity((P::MASK_SIZE + 1) * S::GLEV_L * (P::MASK_SIZE + 1));


        for elem_number in 0..P::MASK_SIZE {
            let key_bit_ = old_key.get_poly_by_index(elem_number)[0];
            let key_bit = Polynomial::<{P::POLINOMIAL_SIZE}>::new_monomial(key_bit_, 0);
            //println!("encrypt_ggsw.message_: {}", message_);
            self.encrypt_glev(&key_bit, &mut ct_data).unwrap();
            // for i in 0..=P::MASK_SIZE {
            //     println!("encrypt_ggsw.ct_data[{i}]: {:?}", ct_data[i]);
            // }
        }


        KeyswitchingKey::from_polynomial_list(from_poly_list::from(ct_data))
    }
}

#[cfg(test)]
proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]
    #[test]
    fn pt_encrypt_invertable(m in any::<[u8; GLWE_Params::<TFHE_test_small_u64>::POLINOMIAL_SIZE]>().prop_map(|v| Polynomial::<{GLWE_Params::<TFHE_test_small_u64>::POLINOMIAL_SIZE}>::new(v.iter().map(|vv| (*vv as u64) << 58).collect()))) {

        let sk: GLWE_secret_key<TFHE_test_small_u64, GLWE_Params<TFHE_test_small_u64>> = GLWE_secret_key::new_random();
        let encrypted: GLWECiphertext<TFHE_test_small_u64, GLWE_Params<TFHE_test_small_u64>> = sk.encrypt(&m);
        let decrypted = sk.decrypt(&encrypted);

        prop_assert_eq!(decrypted.into_iter().map(|v| v>>58).collect::<Vec<u64>>(), m.into_iter().map(|v| v>>58).collect::<Vec<u64>>());
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

        let sk: GLWE_secret_key<TFHE_test_small_u64, GLWE_Params<TFHE_test_small_u64>> = GLWE_secret_key::new_random();

        let encrypted_a: GLWECiphertext<TFHE_test_small_u64, GLWE_Params<TFHE_test_small_u64>> = sk.encrypt(&a);
        let encrypted_b: GLWECiphertext<TFHE_test_small_u64, GLWE_Params<TFHE_test_small_u64>> = sk.encrypt(&b);
        println!("pt_glwe_ct_add 1");
        let sum = &encrypted_a + &encrypted_b;
        println!("pt_glwe_ct_add 2");

        //здесь
        let decrypted_sum = sk.decrypt(&sum);
        println!("pt_glwe_ct_add 3");
        let expected_sum = &a + &b;
        println!("pt_glwe_ct_add 4");

        prop_assert_eq!(decrypted_sum, expected_sum);

    }
}

#[cfg(test)]
proptest! {
    #![proptest_config(ProptestConfig::with_cases(1000))]
    #[test]
    fn pt_glwe_ct_sub(a in any::<[u8; GLWE_Params::<TFHE_test_small_u64>::POLINOMIAL_SIZE]>().prop_map(|v| Polynomial::<{GLWE_Params::<TFHE_test_small_u64>::POLINOMIAL_SIZE}>::new(v.iter().map(|vv| (*vv >> 4) as u64).collect())),
                      b in any::<[u8; GLWE_Params::<TFHE_test_small_u64>::POLINOMIAL_SIZE]>().prop_map(|v| Polynomial::<{GLWE_Params::<TFHE_test_small_u64>::POLINOMIAL_SIZE}>::new(v.iter().map(|vv| (*vv >> 4) as u64).collect())))  {

        let sk: GLWE_secret_key<TFHE_test_small_u64, GLWE_Params<TFHE_test_small_u64>> = GLWE_secret_key::new_random();

        let encrypted_a: GLWECiphertext<TFHE_test_small_u64, GLWE_Params<TFHE_test_small_u64>> = sk.encrypt(&a);
        let encrypted_b: GLWECiphertext<TFHE_test_small_u64, GLWE_Params<TFHE_test_small_u64>> = sk.encrypt(&b);
        println!("pt_glwe_ct_sub 1");
        let diff = &encrypted_a - &encrypted_b;
        println!("pt_glwe_ct_sub 2");



        //здесь
        let decrypted_diff = sk.decrypt(&diff);
        println!("pt_glwe_ct_sub 3");
        let expected_diff = &a - &b;
        println!("pt_glwe_ct_sub 4");

        prop_assert_eq!(decrypted_diff, expected_diff);

    }
}

// pub fn cmux<S: TFHESchema, P: LWE_CT_Params<S>>(cond: &GGSWCiphertext<S, P>, lhs: &GLWECiphertext<S, P>, rhs: &GLWECiphertext<S, P>) -> GLWECiphertext<S, P> {
//     let diff = lhs - rhs;
//     let mul = &cond * &diff;
//     let res = &mul + rhs;
//     diff
// }

#[cfg(test)]
proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]
    #[test]
    fn pt_encrypt_ggsw_callable(m in any::<[u8; GLWE_Params::<TFHE_test_small_u64>::POLINOMIAL_SIZE]>().prop_map(|v| Polynomial::<{GLWE_Params::<TFHE_test_small_u64>::POLINOMIAL_SIZE}>::new(v.iter().map(|vv| *vv as u64).collect()))) {

       let sk: GLWE_secret_key<TFHE_test_small_u64, GLWE_Params<TFHE_test_small_u64>> = GLWE_secret_key::new_random();
       let _encripted: GGSWCiphertext<TFHE_test_small_u64, GLWE_Params<TFHE_test_small_u64>> = sk.encrypt_ggsw(&m);
    //    let decripted = sk.decrypt(&encripted);

    //    prop_assert_eq!(decripted, m);
      // assert_eq!(1,2);
    }
}


#[cfg(test)]
proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]
    #[test]
    fn pt_ggsw_mul_external_expected(a in any::<[u8; GLWE_Params::<TFHE_test_small_u64>::POLINOMIAL_SIZE]>().prop_map(|v| Polynomial::<{GLWE_Params::<TFHE_test_small_u64>::POLINOMIAL_SIZE}>::new(v.iter().map(|vv| (*vv >> 4) as u64).collect())),
                            b in any::<[u8; GLWE_Params::<TFHE_test_small_u64>::POLINOMIAL_SIZE]>().prop_map(|v| Polynomial::<{GLWE_Params::<TFHE_test_small_u64>::POLINOMIAL_SIZE}>::new(v.iter().map(|vv| ((*vv >> 4) as u64) <<56).collect()))) {

        let sk: GLWE_secret_key<TFHE_test_small_u64, GLWE_Params<TFHE_test_small_u64>> = GLWE_secret_key::new_random();

        let encrypted_a: GGSWCiphertext<TFHE_test_small_u64, GLWE_Params<TFHE_test_small_u64>> = sk.encrypt_ggsw(&a);
        let encrypted_b: GLWECiphertext<TFHE_test_small_u64, GLWE_Params<TFHE_test_small_u64>> = sk.encrypt(&b);
        let decrypted_b = sk.decrypt(&encrypted_b);


        let expected_product =  &a * &b;

        let product = &encrypted_a * &encrypted_b;


        let decrypted_product = sk.decrypt(&product);

        prop_assert_eq!(decrypted_product.into_iter().map(|v| v>>56).collect::<Vec<u64>>(), expected_product.into_iter().map(|v| v>>56).collect::<Vec<u64>>());



    }
}

#[cfg(test)]
proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]
    #[test]
    fn pt_ggsw_mul_external_neutral_rhs(a in any::<[u8; GLWE_Params::<TFHE_test_small_u64>::POLINOMIAL_SIZE]>().prop_map(|v| Polynomial::<{GLWE_Params::<TFHE_test_small_u64>::POLINOMIAL_SIZE}>::new(v.iter().map(|vv| (*vv >> 4) as u64).collect()))) {

        let sk: GLWE_secret_key<TFHE_test_small_u64, GLWE_Params<TFHE_test_small_u64>> = GLWE_secret_key::new_random();

        let b = Polynomial::<{GLWE_Params::<TFHE_test_small_u64>::POLINOMIAL_SIZE}>::new_monomial(1 << 56, 0);

        let encrypted_a: GGSWCiphertext<TFHE_test_small_u64, GLWE_Params<TFHE_test_small_u64>> = sk.encrypt_ggsw(&a);
        let encrypted_b: GLWECiphertext<TFHE_test_small_u64, GLWE_Params<TFHE_test_small_u64>> = sk.encrypt(&b);
        let decrypted_b = sk.decrypt(&encrypted_b);


        let expected_product =  (&a * &b).into_iter().map(|v| (v.wrapping_shr(56)) as u8).collect::<Vec<u8>>();

        let product = &encrypted_a * &encrypted_b;


       // let decrypted_product = sk.decrypt(&product);

        let decrypted_product = (sk.decrypt((&product)).into_iter().map(|v| (v.wrapping_shr(56)) as u8).collect::<Vec<u8>>());

        println!("decripted_product: {:?}, {}, {}, {}", decrypted_product[0], decrypted_product[0] == 0 , decrypted_product[0] == 255, expected_product[0] == decrypted_product[0]);

        let decrypted_product_0 = decrypted_product[0].clone();

        println!("decrypted_product_0: {:?}, {}, {}, {}", decrypted_product_0, decrypted_product_0 == 0 , decrypted_product_0== 255, expected_product[0] == decrypted_product_0);



        prop_assert_eq!(expected_product[0], decrypted_product_0);

      //  prop_assert_eq!(decrypted_product_0, expected_product[0]);



    }
}

#[cfg(test)]
proptest! {
    #![proptest_config(ProptestConfig::with_cases(10))]
    #[test]
    fn pt_ggsw_mul_external_absorbing_rhs(a in any::<[u8; GLWE_Params::<TFHE_test_small_u64>::POLINOMIAL_SIZE]>().prop_map(|v| Polynomial::<{GLWE_Params::<TFHE_test_small_u64>::POLINOMIAL_SIZE}>::new(v.iter().map(|vv| (*vv >> 4) as u64).collect()))) {

        let sk: GLWE_secret_key<TFHE_test_small_u64, GLWE_Params<TFHE_test_small_u64>> = GLWE_secret_key::new_random();
        let b = Polynomial::<{GLWE_Params::<TFHE_test_small_u64>::POLINOMIAL_SIZE}>::new_zero();

        let encrypted_a: GGSWCiphertext<TFHE_test_small_u64, GLWE_Params<TFHE_test_small_u64>> = sk.encrypt_ggsw(&a);
        let encrypted_b: GLWECiphertext<TFHE_test_small_u64, GLWE_Params<TFHE_test_small_u64>> = sk.encrypt(&b);
      //  let decrypted_b = sk.decrypt(&encrypted_b);


        let expected_product =  ((&a * &b).into_iter().map(|v| (v.wrapping_shr(56)) as u8).collect::<Vec<u8>>());
        let product = (&encrypted_a) * &encrypted_b;


        let decrypted_product = (sk.decrypt((&product)).into_iter().map(|v| (v.wrapping_shr(56) as u8)).collect::<Vec<u8>>());
        // println!("expected_product: {}", expected_product[0]);
        println!("decrypted_product: {:?}, {}, {}, {}", decrypted_product[0], decrypted_product[0] == 0 , decrypted_product[0] == 255, expected_product[0] == decrypted_product[0]);

        let decrypted_product_0 = decrypted_product[0].clone();
        if expected_product[0] == decrypted_product_0{

            println!("decrypted_product_0: {:?}, {}, {}, {}", decrypted_product_0, decrypted_product_0 == 0 , decrypted_product_0== 255, expected_product[0] == decrypted_product_0);
        }


        // let diff = &encrypted_b - &encrypted_b;
        // let mul = &encrypted_a * &diff;
        // let res = &mul + &encrypted_b;

        if expected_product[0] == decrypted_product_0{

        prop_assert_eq!(expected_product[0], decrypted_product_0);
            println!("decrypted_product_0___: {:?}, {}, {}, {}", decrypted_product_0, decrypted_product_0 == 0 , decrypted_product_0== 255, expected_product[0] == decrypted_product_0);
        }

        // println!("decripted_product: {}, {}, {}", decrypted_product[0], decrypted_product[0] == 0 , decrypted_product[0] == 255);


        assert_eq!(1,2)

    }
}

#[cfg(test)]
proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]
    #[test]
    fn pt_cmux_expected(a in any::<[u8; GLWE_Params::<TFHE_test_small_u64>::POLINOMIAL_SIZE]>().prop_map(|v| Polynomial::<{GLWE_Params::<TFHE_test_small_u64>::POLINOMIAL_SIZE}>::new(v.iter().map(|vv| ((*vv >> 4) as u64)<<60).collect())),
                        b in any::<[u8; GLWE_Params::<TFHE_test_small_u64>::POLINOMIAL_SIZE]>().prop_map(|v| Polynomial::<{GLWE_Params::<TFHE_test_small_u64>::POLINOMIAL_SIZE}>::new(v.iter().map(|vv| ((*vv >> 4) as u64)<<60).collect())),
                        cond__ in any::<u8>().prop_map(|v| v % 2==1)) {

        let cond_ = true;
        let sk: GLWE_secret_key<TFHE_test_small_u64, GLWE_Params<TFHE_test_small_u64>> = GLWE_secret_key::new_random();


        let cond = Polynomial::<{GLWE_Params::<TFHE_test_small_u64>::POLINOMIAL_SIZE}>::new_monomial(if cond_ {1} else {0}, 0);

        let encrypted_cond: GGSWCiphertext<TFHE_test_small_u64, GLWE_Params<TFHE_test_small_u64>> = sk.encrypt_ggsw(&cond);
        let encrypted_a: GLWECiphertext<TFHE_test_small_u64, GLWE_Params<TFHE_test_small_u64>> = sk.encrypt(&a);
        let encrypted_b: GLWECiphertext<TFHE_test_small_u64, GLWE_Params<TFHE_test_small_u64>> = sk.encrypt(&b);
        // let decripted_b = sk.decrypt(&encripted_b);


        let expected_cmux = if cond_ {b} else {a};

        let cmux = cmux(&encrypted_cond, &encrypted_b, &encrypted_a);


        let decrypted_cmux = sk.decrypt(&cmux);

        let decrypted: u64 = decrypted_cmux[0] >> 60;
        let expected: u64 = expected_cmux[0] >> 60;

        if decrypted == expected {
            println!("eq1234");
        }

        assert_eq!(dbg!(decrypted), dbg!(expected));
        // prop_assert_eq!(decrypted_cmux.into_iter().map(|v| v>>60).collect::<Vec<u64>>(), expected_cmux.into_iter().map(|v| v>>60).collect::<Vec<u64>>());
        // assert_eq!(1,2)



    }
}

// #[cfg(test)]
// proptest! {
//     #![proptest_config(ProptestConfig::with_cases(100))]
//     #[test]
//     fn pt_assert_eq(a in any::<[u8; GLWE_Params::<TFHE_test_small_u64>::POLINOMIAL_SIZE]>().prop_map(|v| Polynomial::<{GLWE_Params::<TFHE_test_small_u64>::POLINOMIAL_SIZE}>::new(v.iter().map(|vv| ((*vv >> 4) as u64)<<60).collect())),
//                         b in any::<[u8; GLWE_Params::<TFHE_test_small_u64>::POLINOMIAL_SIZE]>().prop_map(|v| Polynomial::<{GLWE_Params::<TFHE_test_small_u64>::POLINOMIAL_SIZE}>::new(v.iter().map(|vv| ((*vv >> 4) as u64)<<60).collect())),
//                         cond__ in any::<u8>().prop_map(|v| v % 2==1)) {


//         let decrypted: u64 = 2305843009213693952 / 2_u64.pow(60);
//         let expected: u64 = 2305843009213693952 / 2_u64.pow(60);

//         if decrypted == expected {
//             println!("eq1234");
//         }

//         assert_eq!(dbg!(decrypted), dbg!(expected));
//         // prop_assert_eq!(decrypted_cmux.into_iter().map(|v| v>>60).collect::<Vec<u64>>(), expected_cmux.into_iter().map(|v| v>>60).collect::<Vec<u64>>());
//         // assert_eq!(1,2)



//     }
// }

#[cfg(test)]
proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]
    #[test]
    fn pt_create_bootstrapping_key_callable(_ in any::<bool>()) {

        let old: GLWE_secret_key<TFHE_test_small_u64, LWE_Params<TFHE_test_small_u64>> = GLWE_secret_key::new_random();
        let new: GLWE_secret_key<TFHE_test_small_u64, GLWE_Params<TFHE_test_small_u64>> = GLWE_secret_key::new_random();
        let _ = new.create_bootstrapping_key(&old);


    }
}

#[cfg(test)]
proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]
    #[test]
    fn pt_create_keyswitching_key_callable(_ in any::<bool>()) {

        let old: GLWE_secret_key<TFHE_test_small_u64, LWE_Params_after_extraction<TFHE_test_small_u64>> = GLWE_secret_key::new_random();
        let new: GLWE_secret_key<TFHE_test_small_u64, LWE_Params<TFHE_test_small_u64>> = GLWE_secret_key::new_random();
        let _ = new.create_keyswitching_key::<LWE_Params_after_extraction<TFHE_test_small_u64>>(&old);

    }
}


#[cfg(test)]
proptest! {
    #![proptest_config(ProptestConfig::with_cases(1000))]
    #[test]
    fn pt_bootstrapping_expected(message in any::<[u8; LWE_Params::<TFHE_test_small_u64>::POLINOMIAL_SIZE]>().prop_map(|v| Polynomial::<{LWE_Params::<TFHE_test_small_u64>::POLINOMIAL_SIZE}>::new(v.iter().map(|vv| ((*vv >> (8-TFHE_test_small_u64::GLEV_B)) as u64) << (TFHE_test_small_u64::GLWE_Q-TFHE_test_small_u64::GLEV_B) ).collect()))) {

        let sk_old: GLWE_secret_key<TFHE_test_small_u64, LWE_Params<TFHE_test_small_u64>> = GLWE_secret_key::new_random();
        println!("pt_bootstrapping_expected 1, secret_key: {:?}, message: {:?}", sk_old, message);
        let sk_new: GLWE_secret_key<TFHE_test_small_u64, GLWE_Params<TFHE_test_small_u64>> = GLWE_secret_key::new_random();
        println!("pt_bootstrapping_expected 2");
        let mes = Polynomial::<1>::new_monomial(5<<(TFHE_test_small_u64::GLWE_Q-TFHE_test_small_u64::GLEV_B), 0);
        let encrypted_message: GLWECiphertext<TFHE_test_small_u64, LWE_Params<TFHE_test_small_u64>> = sk_old.encrypt(&message);
        println!("pt_bootstrapping_expected 3");
        let bsk: BootstrappingKey<TFHE_test_small_u64, LWE_Params<TFHE_test_small_u64>, GLWE_Params<TFHE_test_small_u64>> = sk_new.create_bootstrapping_key(&sk_old);
        println!("pt_bootstrapping_expected 4");
        let (bootstrapped_message, log_cts): (GLWECiphertext<TFHE_test_small_u64, GLWE_Params<TFHE_test_small_u64>>, Vec<( String, GLWECiphertext<TFHE_test_small_u64, GLWE_Params<TFHE_test_small_u64>>)> ) = bsk.bootstrap(&encrypted_message);
        println!("pt_bootstrapping_expected 5");
        // let decripted_b = sk.decrypt(&encripted_b);

        let test_message = Polynomial::<{GLWE_Params::<TFHE_test_small_u64>::POLINOMIAL_SIZE}>::new_monomial(7<<(TFHE_test_small_u64::GLWE_Q-TFHE_test_small_u64::GLEV_B),0);
        let test_ct = sk_new.encrypt(&test_message);
        let test_message2 = Polynomial::<{GLWE_Params::<TFHE_test_small_u64>::POLINOMIAL_SIZE}>::new_monomial(1<<(TFHE_test_small_u64::GLWE_Q-TFHE_test_small_u64::GLEV_B),0);
        let test_ct2 = sk_new.encrypt(&test_message2);
        // let test_bsk = sk_new.create_bootstrapping_key(&sk_old);
        for i in 0..LWE_Params::<TFHE_test_small_u64>::MASK_SIZE {
            let mul = cmux(&bsk.key[i], &test_ct, &test_ct2);//&test_bsk.key[i] * &test_ct;
            println!("bit times message decrypted: {:?}", sk_new.decrypt(&mul));
        }


        let expected_message = message[0]>>(TFHE_test_small_u64::GLWE_Q-TFHE_test_small_u64::GLEV_B);

        let decrypted_message = sk_new.decrypt(&bootstrapped_message)[0]>>(TFHE_test_small_u64::GLWE_Q-TFHE_test_small_u64::GLEV_B);
        println!("pt_bootstrapping_expected 6, secret_key: {:?}, message: {:?}", sk_old, message);

        for i in 0..log_cts.len() {

            println!("log_cts.{} decrypted: {:?}", log_cts[i].0, sk_new.decrypt(&log_cts[i].1).shr(TFHE_test_small_u64::GLWE_Q-TFHE_test_small_u64::GLEV_B));
        }

        if dbg!(decrypted_message) != dbg!(expected_message) {
            assert_eq!(1,2)
            //prop_assert_eq!(dbg!(decrypted_message), dbg!(expected_message));
        }
        // assert_eq!(decrypted_message, xpected_message);
        // assert_eq!(1,2)



    }
}


#[cfg(test)]
proptest! {
    #![proptest_config(ProptestConfig::with_cases(1000))]
    #[test]
    // fn pt_extract_sample_expected(message in any::<[u8; GLWE_Params::<TFHE_test_small_u64>::POLINOMIAL_SIZE]>().prop_map(|v| Polynomial::<{GLWE_Params::<TFHE_test_small_u64>::POLINOMIAL_SIZE}>::new(v.iter().map(|vv| ((4) as u64) << (TFHE_test_small_u64::GLWE_Q-TFHE_test_small_u64::GLEV_B) ).collect()))) {
        fn pt_extract_sample_expected(message in any::<[u8; GLWE_Params::<TFHE_test_small_u64>::POLINOMIAL_SIZE]>().prop_map(|v| Polynomial::<{GLWE_Params::<TFHE_test_small_u64>::POLINOMIAL_SIZE}>::new(v.iter().map(|vv| ((*vv >> (8-TFHE_test_small_u64::GLEV_B)) as u64) << (TFHE_test_small_u64::GLWE_Q-TFHE_test_small_u64::GLEV_B) ).collect()))) {



        let sk_old: GLWE_secret_key<TFHE_test_small_u64, GLWE_Params<TFHE_test_small_u64>> = GLWE_secret_key::new_random();
        let mut v:Vec<u64> = Vec::with_capacity({GLWE_Params::<TFHE_test_small_u64>::POLINOMIAL_SIZE});
        // for i in 0..GLWE_Params::<TFHE_test_small_u64>::POLINOMIAL_SIZE {
        //     v.push(message_[i]);
        // }
        // v[7] = 7<<(TFHE_test_small_u64::GLWE_Q-TFHE_test_small_u64::GLEV_B);
        // let message = Polynomial::<{GLWE_Params::<TFHE_test_small_u64>::POLINOMIAL_SIZE}>::new(v);
        let encrypted_message: GLWECiphertext<TFHE_test_small_u64, GLWE_Params<TFHE_test_small_u64>> = sk_old.encrypt(&message);
        let (extracted_message, sk_new) = extract_sample::<TFHE_test_small_u64, GLWE_Params<TFHE_test_small_u64>, LWE_Params_after_extraction<TFHE_test_small_u64>>(&encrypted_message, &sk_old, 7);

        let decrypted_extracted_message = sk_new.decrypt(&extracted_message);

        prop_assert_eq!(dbg!(decrypted_extracted_message[0]>>(TFHE_test_small_u64::GLWE_Q-TFHE_test_small_u64::GLEV_B)), dbg!(message[7]>>(TFHE_test_small_u64::GLWE_Q-TFHE_test_small_u64::GLEV_B)));
        // assert_eq!(1,2);


    }
}