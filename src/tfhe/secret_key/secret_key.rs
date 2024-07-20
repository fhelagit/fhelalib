#![allow(non_camel_case_types)]

use std::thread::sleep_ms;

use crate::math::polynomial::polynomial::Polynomial;
use crate::random::random::{rnd_u64_gausean, rnd_u64_uniform_bounded};
use crate::tfhe::ggsw::ggsw::GGSWCiphertext;
use crate::tfhe::glwe::GLWECiphertext;
use crate::tfhe::schemas::{
    from_poly_list, from_u64, GLWE_Params, LWE_CT_Params, LWE_Params, TFHESchema,
    TFHE_test_small_u64,
};
use crate::tfhe::server_key::cmux::cmux;
use crate::tfhe::server_key::server_key::BootstrappingKey;
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
pub struct GLWE_secret_key<S: TFHESchema, P: LWE_CT_Params<S>>(P::SecretKeyContainerType);

impl<S: TFHESchema, P: LWE_CT_Params<S>> GLWE_secret_key<S, P>
where
    [(); P::POLINOMIAL_SIZE]: Sized,
{
    pub fn new_random() -> Self {
        let mut d: Vec<Polynomial<{ P::POLINOMIAL_SIZE }>> = Vec::with_capacity(S::LWE_K);
        for _ in 0..S::LWE_K {
            d.push(Polynomial::<{ P::POLINOMIAL_SIZE }>::new(
                [rnd_u64_uniform_binary(); P::POLINOMIAL_SIZE].to_vec(),
            ));
        }
        GLWE_secret_key::from_scalar_vector(from_poly_list::from(d))
    }

    // #[cfg(test)]
    pub fn from_scalar_vector(data: P::SecretKeyContainerType) -> Self {
        GLWE_secret_key(data)
    }

    pub fn get_poly_by_index(&self, ind: usize) -> Polynomial<{ P::POLINOMIAL_SIZE }> {
        let mut v: Vec<u64> = Vec::with_capacity(P::POLINOMIAL_SIZE);
        for i in 0..P::POLINOMIAL_SIZE {
            v.push(from_u64::to(self.0[ind * P::POLINOMIAL_SIZE + i]));
        }
        Polynomial::<{ P::POLINOMIAL_SIZE }>::new(v)
    }

    fn encrypt(&self, message: &Polynomial<{ P::POLINOMIAL_SIZE }>) -> GLWECiphertext<S, P> {
        // создать полином шума
        let mut e: Vec<u64> = Vec::with_capacity(P::POLINOMIAL_SIZE); //[rnd_u64_gausean() ; P::POLINOMIAL_SIZE].to_vec();
        for _ in 0..P::POLINOMIAL_SIZE {
            e.push(0);//rnd_u64_gausean());
        }
        println!("encrypt.noise: {:?}", e);

        let err = Polynomial::<{ P::POLINOMIAL_SIZE }>::new(e);
        // dbg!(self.0);
        // создать полиномы ашки
        let mut a_s: Vec<Polynomial<{ P::POLINOMIAL_SIZE }>> = Vec::with_capacity(P::MASK_SIZE);
        for _ in 0..P::MASK_SIZE {
            let mut a_i: Vec<u64> = Vec::with_capacity(P::POLINOMIAL_SIZE);
            for _ in 0..P::POLINOMIAL_SIZE {
                a_i.push(rnd_u64_uniform_bounded(1 << 56)); //rnd_u64_uniform());
            }

            a_s.push(Polynomial::new(a_i));
        }
        println!("encrypt.a_s: {:?}", a_s);
        // посчитать мультисумму
        let mut multysum =
            Polynomial::<{ P::POLINOMIAL_SIZE }>::new([0; P::POLINOMIAL_SIZE].to_vec());
        for i in 0..P::MASK_SIZE {
            multysum =
                dbg!(&multysum) + dbg!(&(dbg!(&a_s[i]) * dbg!(&(self.get_poly_by_index(i)))));
        }

        println!("println1");
        // cоздать сдвинутое сообщение
        let _delta = (S::GLWE_Q - S::GLEV_B) as u32;
        let shifted_message = message; // Polynomial::new(message.into_iter().map(|v| v.wrapping_shl(delta)).collect());

        println!("println2");
        // сложить все вместе

        let body = &(dbg!(&multysum) + dbg!(&shifted_message)) + dbg!(&err);
        a_s.push(dbg!(body));

        GLWECiphertext::<S, P>::from_polynomial_list(from_poly_list::from(a_s))
    }

    fn decrypt(&self, ct: &GLWECiphertext<S, P>) -> Polynomial<{ P::POLINOMIAL_SIZE }> {
        println!("decript 1");

        // // посчитать мультисумму
        let mut multysum: Polynomial<{ P::POLINOMIAL_SIZE }> =
            Polynomial::<{ P::POLINOMIAL_SIZE }>::new([0; P::POLINOMIAL_SIZE].to_vec());
        for i in 0..P::MASK_SIZE {
            multysum = &multysum + &(&ct.get_poly_by_index(i) * &(self.get_poly_by_index(i)));
        }

        println!("decript 2");
        let shifted_message = dbg!(&ct.get_poly_by_index(P::MASK_SIZE)) - dbg!(&multysum);
        println!("decript 3");
        let _delta = (S::GLWE_Q - S::GLEV_B) as u32;
        let message = shifted_message; //Polynomial::new(dbg!(shifted_message).into_iter().map(|v| v.wrapping_shr(delta)).collect());

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

    fn encrypt_glev(
        &self,
        message: &Polynomial<{ P::POLINOMIAL_SIZE }>,
        acc: &mut Vec<Polynomial<{ P::POLINOMIAL_SIZE }>>,
    ) -> Result<(), ()> {
        println!("encrypt_glev.message: {}", message);
        for i in 1..=S::GLEV_L {
            let message_ = &Polynomial::new(
                message
                    .into_iter()
                    .map(|v| v << (S::GLWE_Q - S::GLEV_B * i))
                    .collect(),
            );
            println!("encrypt_glev.message_: {}", message_);
            let ct = self.encrypt(message_);
            println!("encrypt_glev.ct: {}", ct);
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
            // let sk_bit = if oldsk[i].coeffs().len() > 0 {oldsk[i].coeffs()[0]} else {ModNumber(0)};
            // println!("sk_bit: {}", sk_bit);
            let mut monom_: Vec<u64> = Vec::with_capacity(P::POLINOMIAL_SIZE);
            for _ in 0..P::POLINOMIAL_SIZE {
                monom_.push(0);
            }
            monom_[0] = old_key.get_poly_by_index(bit_number)[0];
            let monom: Polynomial<{ P::POLINOMIAL_SIZE }> = Polynomial::new(monom_);

            ggsws.push(self.encrypt_ggsw(&monom));
        }

        BootstrappingKey::from_vec(ggsws)
        // Box::new(a)
    }
}

#[cfg(test)]
proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]
    #[test]
    fn pt_encrypt_invertable(m in any::<[u8; GLWE_Params::<TFHE_test_small_u64>::POLINOMIAL_SIZE]>().prop_map(|v| Polynomial::<{GLWE_Params::<TFHE_test_small_u64>::POLINOMIAL_SIZE}>::new(v.iter().map(|vv| *vv as u64).collect()))) {

       let sk: GLWE_secret_key<TFHE_test_small_u64, GLWE_Params<TFHE_test_small_u64>> = dbg!(GLWE_secret_key::new_random());
       let encrypted: GLWECiphertext<TFHE_test_small_u64, GLWE_Params<TFHE_test_small_u64>> = sk.encrypt(dbg!(&m));
       let decrypted = sk.decrypt(dbg!(&encrypted));

       prop_assert_eq!(dbg!(decrypted), dbg!(m));
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

        let encrypted_a: GLWECiphertext<TFHE_test_small_u64, GLWE_Params<TFHE_test_small_u64>> = sk.encrypt(dbg!(&a));
        let encrypted_b: GLWECiphertext<TFHE_test_small_u64, GLWE_Params<TFHE_test_small_u64>> = sk.encrypt(dbg!(&b));
        println!("pt_glwe_ct_add 1");
        let sum = dbg!(&encrypted_a) + &encrypted_b;
        println!("pt_glwe_ct_add 2");

        //здесь
        let decrypted_sum = sk.decrypt(dbg!(&sum));
        println!("pt_glwe_ct_add 3");
        let expected_sum = dbg!(&a) + dbg!(&b);
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

        let sk: GLWE_secret_key<TFHE_test_small_u64, GLWE_Params<TFHE_test_small_u64>> = dbg!(GLWE_secret_key::new_random());

        let encrypted_a: GLWECiphertext<TFHE_test_small_u64, GLWE_Params<TFHE_test_small_u64>> = sk.encrypt(dbg!(&a));
        let encrypted_b: GLWECiphertext<TFHE_test_small_u64, GLWE_Params<TFHE_test_small_u64>> = sk.encrypt(dbg!(&b));
        println!("pt_glwe_ct_sub 1");
        let diff = dbg!(&encrypted_a) - &encrypted_b;
        println!("pt_glwe_ct_sub 2");



        //здесь
        let decrypted_diff = sk.decrypt(dbg!(&diff));
        println!("pt_glwe_ct_sub 3");
        let expected_diff = dbg!(&a) - dbg!(&b);
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


        let expected_product =  (&a * &b).into_iter().map(|v| (v.wrapping_shr(56)) as u8).collect::<Vec<u8>>();

        let product = dbg!(&encrypted_a) * &encrypted_b;


       // let decrypted_product = sk.decrypt(dbg!(&product));

        let decrypted_product = (sk.decrypt((&product)).into_iter().map(|v| (v.wrapping_shr(56)) as u8).collect::<Vec<u8>>());

        println!("decripted_product: {:?}, {}, {}, {}", decrypted_product[0], decrypted_product[0] == 0 , decrypted_product[0] == 255, expected_product[0] == decrypted_product[0]);

        let decrypted_product_0 = decrypted_product[0].clone();

        println!("decrypted_product_0: {:?}, {}, {}, {}", decrypted_product_0, decrypted_product_0 == 0 , decrypted_product_0== 255, expected_product[0] == decrypted_product_0);



        prop_assert_eq!(expected_product[0], decrypted_product_0);

      //  prop_assert_eq!(dbg!(decrypted_product_0), dbg!(expected_product[0]));



    }
}

#[cfg(test)]
proptest! {
    #![proptest_config(ProptestConfig::with_cases(10))]
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
    fn pt_cmux_expected(a in any::<[u8; GLWE_Params::<TFHE_test_small_u64>::POLINOMIAL_SIZE]>().prop_map(|v| Polynomial::<{GLWE_Params::<TFHE_test_small_u64>::POLINOMIAL_SIZE}>::new(v.iter().map(|vv| ((*vv >> 4) as u64)<<56).collect())),
                        b in any::<[u8; GLWE_Params::<TFHE_test_small_u64>::POLINOMIAL_SIZE]>().prop_map(|v| Polynomial::<{GLWE_Params::<TFHE_test_small_u64>::POLINOMIAL_SIZE}>::new(v.iter().map(|vv| ((*vv >> 4) as u64)<<56).collect())),
                        cond in any::<bool>()) {

        let sk: GLWE_secret_key<TFHE_test_small_u64, GLWE_Params<TFHE_test_small_u64>> = dbg!(GLWE_secret_key::new_random());

        let mut cond_: Vec<u64> = Vec::with_capacity(GLWE_Params::<TFHE_test_small_u64>::POLINOMIAL_SIZE);
        for _ in 0..GLWE_Params::<TFHE_test_small_u64>::POLINOMIAL_SIZE {
            cond_.push(0);
        }
        cond_[0] = if cond {1} else {0};

        let encrypted_cond: GGSWCiphertext<TFHE_test_small_u64, GLWE_Params<TFHE_test_small_u64>> = sk.encrypt_ggsw(dbg!(&Polynomial::<{GLWE_Params::<TFHE_test_small_u64>::POLINOMIAL_SIZE}>::new(cond_)));
        let encrypted_a: GLWECiphertext<TFHE_test_small_u64, GLWE_Params<TFHE_test_small_u64>> = sk.encrypt(dbg!(&a));
        let encrypted_b: GLWECiphertext<TFHE_test_small_u64, GLWE_Params<TFHE_test_small_u64>> = sk.encrypt(dbg!(&b));
        // let decripted_b = sk.decrypt(dbg!(&encripted_b));


        let expected_cmux = if cond {b} else {a};

        let cmux = cmux(&encrypted_cond, &encrypted_b, &encrypted_a);


        let decrypted_cmux = sk.decrypt(dbg!(&cmux));
        if dbg!(decrypted_cmux[0]>>56) == dbg!(expected_cmux[0]>>56) {
            println!("eq1234");
        }

        prop_assert_eq!(dbg!(decrypted_cmux.into_iter().map(|v| v>>56).collect::<Vec<u64>>()), dbg!(expected_cmux.into_iter().map(|v| v>>56).collect::<Vec<u64>>()));
        // assert_eq!(1,2)



    }
}

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
    fn pt_bootstrapping_expected(message in any::<[u8; LWE_Params::<TFHE_test_small_u64>::POLINOMIAL_SIZE]>().prop_map(|v| Polynomial::<{LWE_Params::<TFHE_test_small_u64>::POLINOMIAL_SIZE}>::new(v.iter().map(|vv| ((*vv >> 6) as u64) <<58 ).collect()))) {

        let sk_old: GLWE_secret_key<TFHE_test_small_u64, LWE_Params<TFHE_test_small_u64>> = dbg!(GLWE_secret_key::new_random());
        println!("pt_bootstrapping_expected 1");
        let sk_new: GLWE_secret_key<TFHE_test_small_u64, GLWE_Params<TFHE_test_small_u64>> = dbg!(GLWE_secret_key::new_random());
        println!("pt_bootstrapping_expected 2");
        let encrypted_message: GLWECiphertext<TFHE_test_small_u64, LWE_Params<TFHE_test_small_u64>> = sk_old.encrypt(&message);
        println!("pt_bootstrapping_expected 3");
        let bsk: BootstrappingKey<TFHE_test_small_u64, LWE_Params<TFHE_test_small_u64>, GLWE_Params<TFHE_test_small_u64>> = sk_new.create_bootstrapping_key(&sk_old);
        println!("pt_bootstrapping_expected 4");
        let bootstrapped_message: GLWECiphertext<TFHE_test_small_u64, GLWE_Params<TFHE_test_small_u64>> = bsk.bootstrap(dbg!(&encrypted_message));
        println!("pt_bootstrapping_expected 5");
        // let decripted_b = sk.decrypt(dbg!(&encripted_b));


        let expected_message = message[0]>>58;

        let decrypted_message = sk_new.decrypt(dbg!(&bootstrapped_message))[0];
        println!("pt_bootstrapping_expected 6");

        prop_assert_eq!(decrypted_message, expected_message)
        // assert_eq!(1,2)



    }
}