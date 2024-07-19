use crate::tfhe::{
    ggsw::ggsw::GGSWCiphertext,
    schemas::{LWE_CT_Params, TFHESchema},
};
use std::fmt::{self, Display};
use std::str::FromStr;

pub struct BootstrappingKey<S: TFHESchema, P: LWE_CT_Params<S>>(Vec<GGSWCiphertext<S, P>>);

impl<S: TFHESchema, P: LWE_CT_Params<S>> BootstrappingKey<S, P> {
    pub(in crate::tfhe) fn from_vec(data: Vec<GGSWCiphertext<S, P>>) -> Self {
        BootstrappingKey(data)
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
