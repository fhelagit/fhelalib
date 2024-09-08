#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

use tfhela::{
  math::polynomial::polynomial::Polynomial,
  tfhe::{
      glwe::GLWECiphertext,
      schemas::{
          GLWE_Params, LWE_CT_Params, LWE_Params, LWE_Params_after_extraction, TFHESchema,
      },
      secret_key::secret_key::GLWE_secret_key,
      server_key::server_key::{BootstrappingKey, EvaluatingKey},
  },
};

use criterion::{criterion_group, criterion_main, Criterion};
use rand::Rng;

#[derive(Debug, PartialEq, Clone)]
struct MySchema;

impl TFHESchema for MySchema {
    const LWE_K: usize = 500;
    const GLWE_N: usize = 1024;
    const GLWE_K: usize = 1;
    const CT_MODULUS: u64 = u64::MAX;
    const GLWE_Q: usize = 64;
    const GLEV_B: usize = 3;
    const GLEV_L: usize = 10;
    type ScalarType = u64;
    type GLWECTContainerType = Vec<Self::ScalarType>;
    type SecretKeyContainerType = Vec<Self::ScalarType>;
    type PolynomialContainerType = Vec<Self::ScalarType>;
}

fn bench_bootstrap(c: &mut Criterion) {
    let message = Polynomial::<1>::new_monomial(0<<(MySchema::GLWE_Q-MySchema::GLEV_B), 0);

    let sk_old: GLWE_secret_key<MySchema, LWE_Params<MySchema>> = GLWE_secret_key::new_random();
    let sk_new: GLWE_secret_key<MySchema, GLWE_Params<MySchema>> = GLWE_secret_key::new_random();
    let encrypted_message: GLWECiphertext<MySchema, LWE_Params<MySchema>> = sk_old.encrypt(&message);

    let bsk: BootstrappingKey<MySchema, LWE_Params<MySchema>, GLWE_Params<MySchema>> = sk_new.create_bootstrapping_key(&sk_old);
    c.bench_function("bootstrap", |b| b.iter(|| {
        let (_, _): (GLWECiphertext<MySchema, GLWE_Params<MySchema>>, Vec<( String, GLWECiphertext<MySchema, GLWE_Params<MySchema>>)> ) = bsk.bootstrap(&encrypted_message);
    }));
}

criterion_group!(benches, bench_bootstrap);
criterion_main!(benches);
