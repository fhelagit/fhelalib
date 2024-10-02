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
    let mut mul = cond * &diff;
    mul += if_false;
    mul
}

pub fn cmux_<S: TFHESchema, P: LWE_CT_Params<S>>(
    cond: &GGSWCiphertext<S, P>,
    if_true: &GLWECiphertext<S, P>,
    if_false: &GLWECiphertext<S, P>,
) -> (GLWECiphertext<S, P>, Vec<(String, GLWECiphertext<S, P>)>)
where
    [(); P::POLINOMIAL_SIZE]: Sized,
    [(); S::GLEV_B]: Sized,
    [(); S::GLEV_L]: Sized,
    [(); S::GLWE_Q]: Sized,
{
    let mut cts: Vec<(String, GLWECiphertext<S, P>)> = Vec::new();
    cts.push(("if_true".to_string(), if_true.clone()));
    cts.push(("if_false".to_string(), if_false.clone()));
    let diff = if_true - if_false;
    cts.push(("diff".to_string(), diff.clone()));
    let mul = cond * &diff;
    cts.push(("mul".to_string(), mul.clone()));
    let res = &mul + if_false;
    cts.push(("res".to_string(), res.clone()));
    (res, cts)
}
