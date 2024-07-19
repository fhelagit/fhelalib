use crate::tfhe::ggsw::*;
use crate::tfhe::{
    ggsw::ggsw::GGSWCiphertext,
    glwe::GLWECiphertext,
    schemas::{LWE_CT_Params, TFHESchema},
};
use std::ops::Sub;

pub fn cmux<S: TFHESchema, P: LWE_CT_Params<S>>(
    cond: &GGSWCiphertext<S, P>,
    lhs: &GLWECiphertext<S, P>,
    rhs: &GLWECiphertext<S, P>,
) -> GLWECiphertext<S, P>
where
    [(); P::POLINOMIAL_SIZE]: Sized,
    [(); S::GLEV_B]: Sized,
    [(); S::GLEV_L]: Sized,
    [(); S::GLWE_Q]: Sized,
{
    let diff = lhs - rhs;
    let mul = cond * &diff;
    let res = &mul + rhs;
    res
}
