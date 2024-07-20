use crate::{math::{modular::module_switch::mod_switch, polynomial::polynomial::Polynomial}, tfhe::{
    ggsw::ggsw::GGSWCiphertext, glwe::GLWECiphertext, schemas::{from_poly_list, LWE_CT_Params, TFHESchema}, server_key::cmux::cmux
}};
use std::{alloc::Layout, fmt::{self, Display}};
use std::str::FromStr;
use std::marker::PhantomData;

use super::cmux;

pub struct BootstrappingKey<S: TFHESchema, P_lwe: LWE_CT_Params<S>, P_glwe: LWE_CT_Params<S>>{key: Vec<GGSWCiphertext<S, P_glwe>>, phantom: PhantomData<P_lwe>}

impl<S: TFHESchema, P_lwe: LWE_CT_Params<S>, P_glwe: LWE_CT_Params<S>> BootstrappingKey<S, P_lwe, P_glwe> {
    pub(in crate::tfhe) fn from_vec(data: Vec<GGSWCiphertext<S, P_glwe>>) -> Self {
        BootstrappingKey::<S, P_lwe, P_glwe>{key: data, phantom: PhantomData}
    }

    pub fn bootstrap(&self, ct: &GLWECiphertext<S, P_lwe>) -> GLWECiphertext<S, P_glwe> 
    where 
        [(); { P_lwe::POLINOMIAL_SIZE }]: Sized,
        [(); { P_glwe::POLINOMIAL_SIZE }]: Sized,
        [(); S::GLEV_B]: Sized,
        [(); S::GLWE_Q]: Sized,
        [(); S::GLEV_L]: Sized,
     {

      let mut lut_: Vec<Polynomial<{P_glwe::POLINOMIAL_SIZE}>> = Vec::with_capacity(P_glwe::MASK_SIZE+1);
      for _ in 0..P_glwe::MASK_SIZE {
          lut_.push(Polynomial::new_zero());
      }
      let lut__: Vec<u64> = (0..2_u64.pow(S::GLEV_B as u32))
          .flat_map(|e| (0..(P_glwe::POLINOMIAL_SIZE as u64/ (2_u64.pow(S::GLEV_B as u32)))).map(move |_a| (e)))
        .collect();
      lut_.push(Polynomial::new(lut__));


      let mut lut: GLWECiphertext<S, P_glwe> = GLWECiphertext::<S, P_glwe>::from_polynomial_list(from_poly_list::from(lut_));

      let body_ = mod_switch(ct.get_poly_by_index(P_lwe::MASK_SIZE)[0], u64::MAX, P_glwe::POLINOMIAL_SIZE as u64);

      let body = Polynomial::<{P_glwe::POLINOMIAL_SIZE}>::new_monomial(1, P_glwe::POLINOMIAL_SIZE - body_ as usize);
      lut = &lut * &body;

      for i in 0..P_lwe::MASK_SIZE {
        let a_i_ = mod_switch(ct.get_poly_by_index(i)[0], u64::MAX, P_glwe::POLINOMIAL_SIZE as u64);
        let a_i = Polynomial::<{P_glwe::POLINOMIAL_SIZE}>::new_monomial(1, a_i_ as usize);
        let lut_rotated = &lut * &a_i;

        lut = cmux(&self.key[i], &lut_rotated, &lut);

      }

 
      lut
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
