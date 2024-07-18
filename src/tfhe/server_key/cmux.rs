use crate::tfhe::{ggsw::ggsw::GGSWCiphertext, glwe::GLWECiphertext, schemas::{LWE_CT_Params, TFHESchema}};



// pub fn cmux<S: TFHESchema, P: LWE_CT_Params<S>>(cond: &GGSWCiphertext<S, P>, lhs: &GLWECiphertext<S, P>, rhs: &GLWECiphertext<S, P>) -> GLWECiphertext<S, P> {
//     &(&cond * &(lhs - rhs)) + rhs
// }