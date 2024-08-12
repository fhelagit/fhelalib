use crate::{
    math::{
        modular::module_switch::mod_switch,
        polynomial::polynomial::{decompose_polynomial, Polynomial},
    },
    tfhe::{
        ggsw::ggsw::GGSWCiphertext,
        glwe::GLWECiphertext,
        schemas::{from_poly_list, from_u64, LWE_CT_Params, LWE_Params_after_extraction, TFHESchema},
        server_key::{cmux::cmux, extract_sample::extract_sample},
    },
};
use std::{
    alloc::Layout,
    fmt::{self, Display},
};
// use std::str::FromStr;
use std::marker::PhantomData;

pub struct BootstrappingKey<S: TFHESchema, P_lwe: LWE_CT_Params<S>, P_glwe: LWE_CT_Params<S>> {
    pub key: Vec<GGSWCiphertext<S, P_glwe>>,
    phantom: PhantomData<P_lwe>,
}

impl<S: TFHESchema, P_lwe: LWE_CT_Params<S>, P_glwe: LWE_CT_Params<S>>
    BootstrappingKey<S, P_lwe, P_glwe>
{
    pub(in crate::tfhe) fn from_vec(data: Vec<GGSWCiphertext<S, P_glwe>>) -> Self {
        BootstrappingKey::<S, P_lwe, P_glwe> {
            key: data,
            phantom: PhantomData,
        }
    }

    pub fn bootstrap(
        &self,
        ct: &GLWECiphertext<S, P_lwe>,
    ) -> (
        GLWECiphertext<S, P_glwe>,
        Vec<(String, GLWECiphertext<S, P_glwe>)>,
    )
    where
        [(); P_lwe::POLINOMIAL_SIZE]: Sized,
        [(); P_glwe::POLINOMIAL_SIZE]: Sized,
        [(); S::GLEV_B]: Sized,
        [(); S::GLWE_Q]: Sized,
        [(); S::GLEV_L]: Sized,
    {
        // println!("bootstrap 1");

        let message_size_bits = S::GLEV_B as u32;
        let mut cts: Vec<(String, GLWECiphertext<S, P_glwe>)> = Vec::new();
        let mut lut_: Vec<Polynomial<{ P_glwe::POLINOMIAL_SIZE }>> =
            Vec::with_capacity(P_glwe::MASK_SIZE + 1);
        for _ in 0..P_glwe::MASK_SIZE {
            lut_.push(Polynomial::new_zero());
        }
        let lut__: Vec<u64> = (0..2_u64.pow(message_size_bits))
            .flat_map(|e| {
                (0..(P_glwe::POLINOMIAL_SIZE as u64 >> message_size_bits))
                    .map(move |_a| (e << (S::GLWE_Q - S::GLEV_B)))
            })
            .collect();
        lut_.push(Polynomial::<{ P_glwe::POLINOMIAL_SIZE }>::new(lut__));

        // println!("bootstrap 2: lut_ : {:?}", lut_);

        let mut lut: GLWECiphertext<S, P_glwe> =
            GLWECiphertext::<S, P_glwe>::from_polynomial_list(from_poly_list::from(lut_));
        // let lut_shift = Polynomial::new_monomial(1,  P_glwe::POLINOMIAL_SIZE - ((P_glwe::POLINOMIAL_SIZE >> S::GLEV_B) >> 1));
        // lut = &lut * &lut_shift;
        // cts.push(("lut initial".to_string(), lut.clone()));
        // println!("bootstrap 3");

        let body_ = mod_switch(
            ct.get_poly_by_index(P_lwe::MASK_SIZE)[0],
            1 << 64,
            P_glwe::POLINOMIAL_SIZE as u128,
        );
        // println!(
        //     "bootstrap 4: ct.body: {}, switched: {}",
        //     ct.get_poly_by_index(P_lwe::MASK_SIZE)[0],
        //     body_
        // );

        let body = Polynomial::<{ P_glwe::POLINOMIAL_SIZE }>::new_monomial(
            1,
            P_glwe::POLINOMIAL_SIZE - 1 - body_ as usize,
        );
        lut = &lut * &body;
        // println!("bootstrap 5");
        // cts.push(("lut rotated b".to_string(), lut.clone()));

        let mut shift = 1;

        for i in 0..P_lwe::MASK_SIZE {
            let a_i_ = mod_switch(
                ct.get_poly_by_index(i)[0],
                1 << 64,
                P_glwe::POLINOMIAL_SIZE as u128,
            ); //(ct.get_poly_by_index(i)[0] >> (64-7+3)) << 3;//mod_switch(ct.get_poly_by_index(i)[0], 1<<64, P_glwe::POLINOMIAL_SIZE as u128);

            let a_i = Polynomial::<{ P_glwe::POLINOMIAL_SIZE }>::new_monomial(1, a_i_ as usize);
            shift = 0;
            // println!("bootstrap 6");
            let lut_rotated = &lut * &a_i;
            if shift != 0 {
                let lut_rotated =
                    &lut * &Polynomial::<{ P_glwe::POLINOMIAL_SIZE }>::new_monomial(1, 1);
            }

            shift = 0;
            // cts.push((
            //     format!("lut rotated  a[{i}]").to_string(),
            //     lut_rotated.clone(),
            // ));
            // println!(
            //     "bootstrap 7: ct.a[i]: {}, switched: {}",
            //     ct.get_poly_by_index(i)[0],
            //     a_i_
            // );

            lut = cmux(&self.key[i], &lut_rotated, &lut.clone());
            // println!("bootstrap 7/5: lut[{i}]: {}, cmux: {}", lut,  cmux(&self.key[i], &lut_rotated, &lut.clone()));
            // cts.push((format!("lut after cmux[{i}]").to_string(), lut.clone()));
        }
        // println!("bootstrap 8");

        (lut, cts)
    }
}
pub struct KeyswitchingKey<S: TFHESchema, P_lwe_old: LWE_CT_Params<S>, P_lwe: LWE_CT_Params<S>> {
    pub key: P_lwe::ContainerType,
    phantom1: PhantomData<P_lwe>,
    phantom2: PhantomData<P_lwe_old>,
}

impl<S: TFHESchema, P_lwe_old: LWE_CT_Params<S>, P_lwe: LWE_CT_Params<S>>
    KeyswitchingKey<S, P_lwe_old, P_lwe>
{
    pub fn from_polynomial_list(data: P_lwe::ContainerType) -> Self {
        KeyswitchingKey {
            key: data,
            phantom1: PhantomData,
            phantom2: PhantomData,
        }
    }

    pub fn get_poly_by_index(&self, ind: usize) -> Polynomial<{ P_lwe::POLINOMIAL_SIZE }> {
        let mut v = Polynomial::<{ P_lwe::POLINOMIAL_SIZE }>::new_zero();
        for i in 0..P_lwe::POLINOMIAL_SIZE {
            v[i] = from_u64::to(self.key[ind * P_lwe::POLINOMIAL_SIZE + i]);
        }
        v
    }

    pub fn switch_key(&self, ct: &GLWECiphertext<S, P_lwe_old>) -> GLWECiphertext<S, P_lwe>
    where
        [(); { P_lwe::POLINOMIAL_SIZE }]: Sized,
        [(); { P_lwe_old::POLINOMIAL_SIZE }]: Sized,
        [(); S::GLEV_B]: Sized,
        [(); S::GLEV_L]: Sized,
        [(); S::GLWE_Q]: Sized,
    {
        assert_eq!(P_lwe::POLINOMIAL_SIZE, 1);
        assert_eq!(P_lwe_old::POLINOMIAL_SIZE, 1);
        let mut acc: Vec<Polynomial<{ P_lwe::POLINOMIAL_SIZE }>> =
            Vec::with_capacity(P_lwe::MASK_SIZE + 1);
        for _ in 0..=P_lwe::MASK_SIZE {
            acc.push(Polynomial::<{ P_lwe::POLINOMIAL_SIZE }>::new_zero())
        }
        // println!("switch_key 1");
        for glev_number in 0..P_lwe_old::MASK_SIZE {
            // println!("switch_key 2. glev_number: {glev_number}");
            let dec = decompose_polynomial::<
                { S::GLWE_Q },
                { S::GLEV_L },
                { S::GLEV_B },
                { P_lwe_old::POLINOMIAL_SIZE },
            >(ct.get_poly_by_index(glev_number));
            // println!("mul_ext: 2, dec: {:?}", dec);
            let offset_glev = glev_number * (S::GLEV_L * (P_lwe::MASK_SIZE + 1));

            for glwe_number in 0..S::GLEV_L {
                let offset_glwe = glwe_number * (P_lwe::MASK_SIZE + 1);

                for poly_number in 0..=P_lwe::MASK_SIZE {
                    // println!("mul_ext: 3, get_poly_by_index offset_glev: {}, offset_glwe: {}, poly_number: {}, self[]: {:?}, dec[]: {:?}: ", offset_glev, offset_glwe, poly_number, &self.get_poly_by_index(offset_glev+offset_glwe+poly_number), &dec[glwe_number]);
                    // println!("switch_key 3. offset_glev + offset_glwe + poly_number: {}", offset_glev + offset_glwe + poly_number);
                    acc[poly_number] = &acc[poly_number]
                        + &(&dec[glwe_number].swicth_order::<{ P_lwe::POLINOMIAL_SIZE }>()
                            * &self.get_poly_by_index(offset_glev + offset_glwe + poly_number));
                }
            }
        }

        // 330 != 11*1*3*128

        let mut b_ct: Vec<Polynomial<{ P_lwe::POLINOMIAL_SIZE }>> =
            Vec::with_capacity(P_lwe::MASK_SIZE + 1);
        for _ in 0..P_lwe::MASK_SIZE {
            b_ct.push(Polynomial::new_zero());
        }
        // println!("switch_key 4");
        b_ct.push(
            ct.get_poly_by_index(P_lwe_old::MASK_SIZE)
                .swicth_order::<{ P_lwe::POLINOMIAL_SIZE }>(),
        );
        &GLWECiphertext::from_polynomial_list(from_poly_list::from(b_ct))
            - &GLWECiphertext::from_polynomial_list(from_poly_list::from(acc))
    }
}


pub struct EvaluatingKey<S: TFHESchema, P_lwe: LWE_CT_Params<S>, P_glwe: LWE_CT_Params<S>> {
    pub ksk: KeyswitchingKey<S, LWE_Params_after_extraction<S>, P_lwe>,
    pub bsk: BootstrappingKey<S, P_lwe, P_glwe>,
    phantom1: PhantomData<P_lwe>,
    phantom2: PhantomData<P_glwe>,
}

impl<S: TFHESchema, P_lwe: LWE_CT_Params<S>, P_glwe: LWE_CT_Params<S>>
EvaluatingKey<S, P_lwe, P_glwe>
{
    pub fn new(bsk: BootstrappingKey<S, P_lwe, P_glwe>, ksk: KeyswitchingKey<S, LWE_Params_after_extraction<S>, P_lwe>) -> Self {
        EvaluatingKey {
            bsk: bsk,
            ksk: ksk,
            phantom1: PhantomData,
            phantom2: PhantomData,
        }
    }

    pub fn eval(&self, ct: &GLWECiphertext<S, P_lwe>) -> GLWECiphertext<S, P_lwe>
    where
        [(); { P_lwe::POLINOMIAL_SIZE }]: Sized,
        [(); { P_glwe::POLINOMIAL_SIZE }]: Sized,
        [(); { LWE_Params_after_extraction::<S>::POLINOMIAL_SIZE }]: Sized,
        [(); S::GLEV_B]: Sized,
        [(); S::GLEV_L]: Sized,
        [(); S::GLWE_Q]: Sized,
    {
        assert_eq!(P_lwe::POLINOMIAL_SIZE, 1);

        let (bootstrapped_message, _): (GLWECiphertext<S, P_glwe>, Vec<( String, GLWECiphertext<S, P_glwe>)> ) = self.bsk.bootstrap(ct);


        let extracted_message = extract_sample::<S, P_glwe, LWE_Params_after_extraction<S>>(&bootstrapped_message, 0);

        let keyswitched_message = self.ksk.switch_key(&extracted_message);

        keyswitched_message

    }
}

// impl<S: TFHESchema, P: LWE_CT_Params<S>> Display
//     for BootstrappingKey<S, P>
// {
//     fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
//         write!(
//             formatter,
//             "{}",
//             serde_json::to_string(&self.0).unwrap()
//             //self.0
//         )
//         .unwrap();
//         Ok(())
//     }
// }

// impl<S: TFHESchema, P: LWE_CT_Params<S>> FromStr
//     for GLWECipBootstrappingKeyhertext<S, P>
// {
//     type Err = &'static str;

//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         let data: P::ContainerType = serde_json::from_str(s).unwrap();
//         Ok(GLWECiphertext::from_polynomial_list(data))
//     }
// }
