use crate::math::polynomial::polynomial::Polynomial;
use crate::tfhe::ggsw::*;
use crate::tfhe::schemas::from_poly_list;
use crate::tfhe::{
    ggsw::ggsw::GGSWCiphertext,
    glwe::GLWECiphertext,
    schemas::{LWE_CT_Params, TFHESchema},
    secret_key::secret_key::GLWE_secret_key,
};
use std::ops::Sub;

pub fn extract_sample<S: TFHESchema, P_old: LWE_CT_Params<S>,  P_new: LWE_CT_Params<S>>(
    ct: &GLWECiphertext<S, P_old>,
    sample_position: usize,
)  -> GLWECiphertext<S, P_new>
    where 
        [(); P_old::POLINOMIAL_SIZE]: Sized 
    {
    assert_eq!(P_new::POLINOMIAL_SIZE, 1);


    let mut a:Vec<Polynomial<1>> = Vec::with_capacity(P_old::POLINOMIAL_SIZE*P_old::MASK_SIZE);
    for _ in 0..(P_old::POLINOMIAL_SIZE*P_old::MASK_SIZE) {
        a.push(Polynomial::<1>::new_zero());
    }

    // let mut new_mask = [Poly::<1>::new([ModNumber(0)].to_vec()); BSK_POLY_SIZE_MUL_K];
    for i in 0..P_old::MASK_SIZE {
        for j in 0..P_old::POLINOMIAL_SIZE {
            a[P_old::POLINOMIAL_SIZE * i + j] = if j <= sample_position {
                let val = ct.get_poly_by_index(i)[sample_position - j];
                Polynomial::new_monomial(val, 0)
            } else {
                // так как у я не использую отрицательные коэффициенты полинома, мне не нужно умножать здесь коэффициент на (-1)
                let val = ct.get_poly_by_index(i)[sample_position + P_old::POLINOMIAL_SIZE - j];
                Polynomial::new_monomial(val, 0)
            }
        }
    }
    let val = ct.get_poly_by_index(P_old::MASK_SIZE);
    a.push(Polynomial::new_monomial(val[sample_position], 0));

    let new_ct = GLWECiphertext::<S, P_new>::from_polynomial_list(from_poly_list::from(a));

    new_ct


}

