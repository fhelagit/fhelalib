use std::fmt::{self, Display};
use std::str::FromStr;
use std::ops;
use crate::math::polynomial::polynomial::Polynomial;
extern crate serde_json;

#[cfg(test)]
use proptest::prelude::*;



// #[cfg(test)]
// use proptest_derive::Arbitrary;
use crate::tfhe::schemas::{TFHE_test_medium_u64, TFHE_test_small_u64, LWE_CT_Params, LWE_Params, GLWE_Params, from_u64, TFHESchema, from_poly_list};

// #[cfg(test)]

// use crate::tfhe::secret_key::GLWE_secret_key;

#[derive(Debug, PartialEq)]
pub struct GLWECiphertext<S: TFHESchema, P: LWE_CT_Params<S>>(P::ContainerType);

impl<S: TFHESchema, P: LWE_CT_Params<S>> GLWECiphertext<S, P>
{
    // fn new(data: Box<[u64; Polynomialsize*Masksize]>) -> Self
    // where
    //   [(); Masksize+1]: Sized
    // {
    //   GLWECiphertext(data)
    // }

    pub fn from_polynomial_list(data: P::ContainerType) -> Self {
        GLWECiphertext(data)
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
    for GLWECiphertext<S, P>

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
    for GLWECiphertext<S, P>
{
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let data: P::ContainerType = serde_json::from_str(s).unwrap();
        Ok(GLWECiphertext::from_polynomial_list(data))
    }
}

#[test]
fn test_glwe_to_str_serialization() {
    // todo make iterative, make random
    let a = [0; 96].to_vec();
    let ct: GLWECiphertext<TFHE_test_small_u64, LWE_Params<TFHE_test_small_u64>> = GLWECiphertext::from_polynomial_list(a);

    let serialized = ct.to_string();
    let deserialized: GLWECiphertext<TFHE_test_small_u64, LWE_Params<TFHE_test_small_u64>> = FromStr::from_str(&serialized).unwrap();
    assert_eq!(ct.0, deserialized.0);
}

#[cfg(test)]
proptest! {
    #![proptest_config(ProptestConfig::with_cases(1000))]
    #[test]
    fn pt_glwe_ct_str_serialization(ct in any::<[u64; GLWE_Params::<TFHE_test_medium_u64>::POLINOMIAL_SIZE*(GLWE_Params::<TFHE_test_medium_u64>::MASK_SIZE+1)]>()
        .prop_map(|v| GLWECiphertext::<TFHE_test_medium_u64, GLWE_Params<TFHE_test_medium_u64>>::from_polynomial_list(v.to_vec()))) {

        let serialized = ct.to_string();
        let deserialized: GLWECiphertext<TFHE_test_medium_u64, GLWE_Params<TFHE_test_medium_u64>> = FromStr::from_str(&serialized).unwrap();
        prop_assert_eq!(ct, deserialized);

    }
}

// ops
impl<S: TFHESchema, P: LWE_CT_Params<S>> ops::Add<&GLWECiphertext<S, P>> for &GLWECiphertext<S, P> where [(); {P::POLINOMIAL_SIZE}]: Sized {
    type Output = GLWECiphertext<S, P>;

    fn add(self, rhs: &GLWECiphertext<S, P>) -> GLWECiphertext<S, P> {
        let mut sums: Vec<Polynomial<{P::POLINOMIAL_SIZE}>> = Vec::with_capacity(P::MASK_SIZE+1);

        println!("P::MASK_SIZE: {}", P::MASK_SIZE);
        for i in 0..(P::MASK_SIZE+1) {
            sums.push(&self.get_poly_by_index(i) + &rhs.get_poly_by_index(i));
        }
        GLWECiphertext::<S, P>::from_polynomial_list(from_poly_list::from(sums))
    }
}

#[cfg(test)]
proptest! {
    #![proptest_config(ProptestConfig::with_cases(1000))]
    #[test]
    fn pt_glwe_ct_add_commutative(a in any::<[u64; GLWE_Params::<TFHE_test_small_u64>::POLINOMIAL_SIZE*(GLWE_Params::<TFHE_test_small_u64>::MASK_SIZE+1)]>()
    .prop_map(|v| GLWECiphertext::<TFHE_test_small_u64, GLWE_Params<TFHE_test_small_u64>>::from_polynomial_list(v.to_vec())), 
    b in any::<[u64; GLWE_Params::<TFHE_test_small_u64>::POLINOMIAL_SIZE*(GLWE_Params::<TFHE_test_small_u64>::MASK_SIZE+1)]>()
    .prop_map(|v| GLWECiphertext::<TFHE_test_small_u64, GLWE_Params<TFHE_test_small_u64>>::from_polynomial_list(v.to_vec())))  {

        prop_assert_eq!(dbg!(&a) + dbg!(&b), &b + &a);

    }
}

// #[cfg(test)]
// proptest! {
//     #![proptest_config(ProptestConfig::with_cases(1000))]
//     #[test]
//     fn pt_glwe_ct_add(a in any::<[u8; GLWE_Params::<TFHE_test_small_u64>::POLINOMIAL_SIZE]>().prop_map(|v| Polynomial::<{GLWE_Params::<TFHE_test_small_u64>::POLINOMIAL_SIZE}>::new(v.iter().map(|vv| *vv as u64).collect())), 
//                       b in any::<[u8; GLWE_Params::<TFHE_test_small_u64>::POLINOMIAL_SIZE]>().prop_map(|v| Polynomial::<{GLWE_Params::<TFHE_test_small_u64>::POLINOMIAL_SIZE}>::new(v.iter().map(|vv| *vv as u64).collect())))  {

//         let sk: GLWE_secret_key<TFHE_test_small_u64, GLWE_Params<TFHE_test_small_u64>> = dbg!(GLWE_secret_key::new_random());

//         let encripted_a: GLWECiphertext<TFHE_test_small_u64, GLWE_Params<TFHE_test_small_u64>> = sk.encript(dbg!(&a));
//         let encripted_b: GLWECiphertext<TFHE_test_small_u64, GLWE_Params<TFHE_test_small_u64>> = sk.encript(dbg!(&b));
//         let sum = &encripted_a + &encripted_b;
//         let decripted_sum = sk.decript(dbg!(&sum));
//         let expected_sum = &a + &b;

//         prop_assert_eq!(decripted_sum, expected_sum);

//     }
// }
