use crate::tfhe::{
    ggsw::ggsw::GGSWCiphertext, glwe::GLWECiphertext, schemas::{LWE_CT_Params, TFHESchema}
};
use std::fmt::{self, Display};
use std::str::FromStr;
use std::marker::PhantomData;

pub struct BootstrappingKey<S: TFHESchema, P_lwe: LWE_CT_Params<S>, P_glwe: LWE_CT_Params<S>>{key: Vec<GGSWCiphertext<S, P_glwe>>, phantom: PhantomData<P_lwe>}

impl<S: TFHESchema, P_lwe: LWE_CT_Params<S>, P_glwe: LWE_CT_Params<S>> BootstrappingKey<S, P_lwe, P_glwe> {
    pub(in crate::tfhe) fn from_vec(data: Vec<GGSWCiphertext<S, P_glwe>>) -> Self {
        BootstrappingKey::<S, P_lwe, P_glwe>{key: data, phantom: PhantomData}
    }

    // pub fn bootstrap(ct: GLWECiphertext<S, P_lwe>) -> GLWECiphertext<S, P_glwe> {

    // }

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
