#![feature(generic_const_exprs)]

use tfhela::{
    math::polynomial::polynomial::Polynomial,
    tfhe::{
        glwe::GLWECiphertext,
        schemas::{
            GLWE_Params, LWE_CT_Params, LWE_Params, LWE_Params_after_extraction, TFHESchema,
            TFHE_test_small_u64,
        },
        secret_key::secret_key::GLWE_secret_key,
        server_key::server_key::{BootstrappingKey, EvaluatingKey},
    },
};

fn main() {
    let message =
        Polynomial::<1>::new_monomial(50 << (TFHE_schema::GLWE_Q - TFHE_schema::GLEV_B), 0);

    let sk_old: GLWE_secret_key<TFHE_schema, LWE_Params<TFHE_schema>> =
        GLWE_secret_key::new_random();

    let sk_new: GLWE_secret_key<TFHE_schema, GLWE_Params<TFHE_schema>> =
        GLWE_secret_key::new_random();
    let extracted_key = sk_new.extract_key::<LWE_Params_after_extraction<TFHE_schema>>();
    let ksk =
        sk_old.create_keyswitching_key::<LWE_Params_after_extraction<TFHE_schema>>(&extracted_key);
    let bsk: BootstrappingKey<TFHE_schema, LWE_Params<TFHE_schema>, GLWE_Params<TFHE_schema>> =
        sk_new.create_bootstrapping_key(&sk_old);
    let eval_key = EvaluatingKey::new(bsk, ksk);

    let encrypted_message: GLWECiphertext<TFHE_schema, LWE_Params<TFHE_schema>> =
        sk_old.encrypt(&message);

    let f = |v: u64| v;

    let evaluated_message = eval_key.eval(&encrypted_message, &f);

    let expected_message = message.shr(TFHE_schema::GLWE_Q - TFHE_schema::GLEV_B)[0];

    let decrypted_message = sk_old
        .decrypt(&evaluated_message)
        .shr(TFHE_schema::GLWE_Q - TFHE_schema::GLEV_B)[0];

    assert_eq!(decrypted_message, expected_message);
    println!("Ok")
}

#[derive(Debug, PartialEq, Clone)]
struct TFHE_schema;

impl TFHESchema for TFHE_schema {
    const LWE_K: usize = 3;
    const GLWE_N: usize = 256;
    const GLWE_K: usize = 1;
    const CT_MODULUS: u64 = u64::MAX;
    const GLWE_Q: usize = 64;
    const GLEV_B: usize = 6;
    const GLEV_L: usize = 3;
    type ScalarType = u64;
    type GLWECTContainerType = Vec<Self::ScalarType>;
    type SecretKeyContainerType = Vec<Self::ScalarType>;
    type PolynomialContainerType = Vec<Self::ScalarType>;
}

struct Char_ct<S: TFHESchema, P: LWE_CT_Params<S>>(GLWECiphertext<S, P>);

struct String_ct<S: TFHESchema, P: LWE_CT_Params<S>>(Vec<Char_ct<S, P>>);

struct SecretKey<S: TFHESchema, P: LWE_CT_Params<S>>(GLWE_secret_key<S, P>);
impl<S: TFHESchema, P: LWE_CT_Params<S>> SecretKey<S, P>
where
    [(); P::POLINOMIAL_SIZE]: Sized,
    [(); GLWE_Params::<S>::POLINOMIAL_SIZE]: Sized,
    [(); LWE_Params_after_extraction::<S>::POLINOMIAL_SIZE]: Sized,
{
    pub fn new() -> Self {
        SecretKey(GLWE_secret_key::<S, P>::new_random())
    }

    pub fn encrypt_string(&self, s: String) -> String_ct<S, P> {
        let mut v: Vec<Char_ct<S, P>> = Vec::with_capacity(s.len());
        for i in 0..s.len() {
            let c: u64 = s.chars().nth(i).unwrap() as u64;
            v.push(self.encrypt_char(c));
        }
        String_ct(v)
    }

    pub fn encrypt_char(&self, s: u64) -> Char_ct<S, P> {
        Char_ct(
            self.0
                .encrypt(&Polynomial::<{ P::POLINOMIAL_SIZE }>::new_monomial(s, 0)),
        )
    }

    pub fn make_eval_key(&self) -> EvalKey<S, P, GLWE_Params<S>> {
        let sk_new: GLWE_secret_key<S, GLWE_Params<S>> = GLWE_secret_key::<S, GLWE_Params<S>>::new_random();
        let extracted_key = sk_new.extract_key::<LWE_Params_after_extraction<S>>();
        let ksk = self
            .0
            .create_keyswitching_key::<LWE_Params_after_extraction<S>>(&extracted_key);
        let bsk: BootstrappingKey<S, P, GLWE_Params<S>> = sk_new.create_bootstrapping_key(&self.0);
        let eval_key = EvaluatingKey::new(bsk, ksk);
        EvalKey(eval_key)
    }
}

struct EvalKey<S: TFHESchema, P_lwe: LWE_CT_Params<S>, P_glwe: LWE_CT_Params<S>>(
    EvaluatingKey<S, P_lwe, P_glwe>,
);
impl<S: TFHESchema, P_lwe: LWE_CT_Params<S>, P_glwe: LWE_CT_Params<S>> EvalKey<S, P_lwe, P_glwe> {}
