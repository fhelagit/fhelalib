use crate::tfhe::{GLEV_B, GLWE_Q};
use crate::{
    math::polynomial::polynomial::Polynomial, random::random::rnd_u64_uniform_binary,
    tfhe::glwe::GLWECiphertext,
};

pub struct GLWE_secret_key<const Polynomialsize: usize, const Masksize: usize>(
    [u64; Polynomialsize * Masksize],
)
where
    [(); Polynomialsize * Masksize]: Sized;

impl<const Polynomialsize: usize, const Masksize: usize> GLWE_secret_key<Polynomialsize, Masksize>
where
    [(); Polynomialsize * Masksize]: Sized,
{
    pub fn new_random() -> Self {
        let mut d: [u64; Polynomialsize * Masksize] = [0; Polynomialsize * Masksize];
        for i in 0..Polynomialsize * Masksize {
            d[i] = rnd_u64_uniform_binary();
        }
        GLWE_secret_key(d)
    }

    #[cfg(test)]
    pub fn from_polynomial_list(data: [u64; Polynomialsize * Masksize]) -> Self {
        GLWE_secret_key(data)
    }

    //   pub fn encript(
    //     m: &Poly<POLY_SIZE>,
    //     GLWESecretKey(ss): &GLWESecretKey<POLY_SIZE, K>,
    //     custom_delta: Option<u64>,
    // ) -> GLWECiphertext<POLY_SIZE, K> {
    //     //let err = random_uniform_binary();
    //     let err = zero_poly::<POLY_SIZE>();
    //     // dbg!(e);
    //     let delta: u64 = match custom_delta {
    //         Some(d) => d,
    //         _ => 2_u64.pow((GLWE_Q - GLEV_B) as u32),
    //     };
    //     // dbg!("enc delta: {}", delta);
    //     // dbg!(delta);
    //     let dm = m * delta  as usize;
    //     // dbg!(dm);
    //     let mut mask_vec: Vec<Poly<POLY_SIZE>> = Vec::new();

    //     for _ in 0..K {
    //         mask_vec.push( //zero_poly::<POLY_SIZE>());
    //         random_uniform_glwe_q();
    //     }

    //     let mask: [Poly<POLY_SIZE>; K] = mask_vec.try_into().unwrap();

    //     let mask2 = mask.clone();
    //     //     .iter()
    //     //     .map(|_| (random_uniform_glwe_q()))
    //     //     .collect::<Vec<Poly<POLY_SIZE>>>()
    //     //     .try_into()
    //     //     .unwrap();
    //     // dbg!(mask);
    //     let ListOfPoly(mul_mask_skey) = &ListOfPoly(mask2) * &ListOfPoly(ss.clone());
    //     let mul_sum_mask_skey = mul_mask_skey
    //         .iter()
    //         .fold(Poly::<POLY_SIZE>::new([ModNumber(0); POLY_SIZE].to_vec()), |acc, e| {
    //             acc + e
    //         });

    //     let body = &(mul_sum_mask_skey + &dm) + &err;
    //     GLWECiphertext { mask, body }
    // }
    // fn encript(self, message: Polynomial<Polynomialsize>) -> GLWECiphertext {
    //   let err = zero_poly::<POLY_SIZE>();
    //   let delta: u64 = 2_u64.pow((GLWE_Q - GLEV_B) as u32);
    //   GLWECiphertext::from_polynomial_list(data)
    // }
}
