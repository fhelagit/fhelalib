use crate::tfhe::{
    ggsw::ggsw::GGSWCiphertext, glwe::GLWECiphertext, schemas::{LWE_CT_Params, TFHESchema}
};
use std::{alloc::Layout, fmt::{self, Display}};
use std::str::FromStr;
use std::marker::PhantomData;

pub struct BootstrappingKey<S: TFHESchema, P_lwe: LWE_CT_Params<S>, P_glwe: LWE_CT_Params<S>>{key: Vec<GGSWCiphertext<S, P_glwe>>, phantom: PhantomData<P_lwe>}

impl<S: TFHESchema, P_lwe: LWE_CT_Params<S>, P_glwe: LWE_CT_Params<S>> BootstrappingKey<S, P_lwe, P_glwe> {
    pub(in crate::tfhe) fn from_vec(data: Vec<GGSWCiphertext<S, P_glwe>>) -> Self {
        BootstrappingKey::<S, P_lwe, P_glwe>{key: data, phantom: PhantomData}
    }

    pub fn bootstrap(ct: GLWECiphertext<S, P_lwe>) -> ()
    where [(); { P_lwe::POLINOMIAL_SIZE }]: Sized
    //  -> GLWECiphertext<S, P_glwe> 
     {

      // create Lut
      // let lut_: Vec<u64> = Vec::with_capacity(P_glwe::POLINOMIAL_SIZE);
      let lut_: Vec<u64> = (0..2_u64.pow(S::GLEV_B as u32))
           //  .chain([7,6,5,4].iter().map(|e| *e))
            //(0..2_u64.pow(GLEV_B as u32)).map(|e| if e*2 >= 2_u64.pow(GLEV_B as u32) {u64::wrapping_neg(e)} else {e})
           //  .chain((0..2_u64.pow(GLEV_B as u32-1)).map(|e| u64::wrapping_neg(e)))
             // (0..2_u64.pow(GLEV_B as u32-1)).map(|e| u64::wrapping_neg(e))
             // .chain((0..2_u64.pow(GLEV_B as u32-1)))
             // .iter()
            // .map(|e| e.wrapping_sub(2_u64.pow(GLEV_B as u32-1)))
          .flat_map(|e| (0..(P_glwe::POLINOMIAL_SIZE as u64/ (2_u64.pow(S::GLEV_B as u32)))).map(move |_a| (e)))
          .collect();
      // превратить в штфртекст

      // rotate -b
      let body = ct.get_poly_by_index(P_lwe::MASK_SIZE)[0];
      // превратить в моном
      // умножить на лют

      // rotate a
      for i in 0..P_lwe::MASK_SIZE {
        let a_i = ct.get_poly_by_index(i)[0];
        // превратить в моном
        // умножить на лют

        // сделать cmux

      }

 
      // return 
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