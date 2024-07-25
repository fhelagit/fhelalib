use crate::tfhe::ggsw::*;
use crate::tfhe::{
    ggsw::ggsw::GGSWCiphertext,
    glwe::GLWECiphertext,
    schemas::{LWE_CT_Params, TFHESchema},
};

pub fn cmux<S: TFHESchema, P: LWE_CT_Params<S>>(
    cond: &GGSWCiphertext<S, P>,
    if_true: &GLWECiphertext<S, P>,
    if_false: &GLWECiphertext<S, P>,
) -> GLWECiphertext<S, P>
where
    [(); P::POLINOMIAL_SIZE]: Sized,
    [(); S::GLEV_B]: Sized,
    [(); S::GLEV_L]: Sized,
    [(); S::GLWE_Q]: Sized,
{
    let diff = if_true - if_false;
    let mul = cond * &diff;
    let res = &mul + if_false;
    res
}
