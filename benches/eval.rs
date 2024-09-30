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
    const MESSAGE_SPACE_SIZE: usize = 5;
    const LWE_K: usize = 500;
    const GLWE_N: usize = 2048;
    const GLWE_K: usize = 1;
    const CT_MODULUS: u64 = u64::MAX;
    const GLWE_Q: usize = 64;
    const GLEV_B: usize = 10;
    const GLEV_L: usize = 3;
    type ScalarType = u64;
    type GLWECTContainerType = Vec<Self::ScalarType>;
    type SecretKeyContainerType = Vec<Self::ScalarType>;
    type PolynomialContainerType = Vec<Self::ScalarType>;
}

fn bench_bootstrap(c: &mut Criterion) {
    let message = Polynomial::<1>::new_monomial(1<<(MySchema::GLWE_Q-MySchema::GLEV_B), 0);

    let sk_old: GLWE_secret_key<MySchema, LWE_Params<MySchema>> = GLWE_secret_key::new_random();
    let sk_new: GLWE_secret_key<MySchema, GLWE_Params<MySchema>> = GLWE_secret_key::new_random();
    let encrypted_message: GLWECiphertext<MySchema, LWE_Params<MySchema>> = sk_old.encrypt(&message);

    let bsk: BootstrappingKey<MySchema, LWE_Params<MySchema>, GLWE_Params<MySchema>> = sk_new.create_bootstrapping_key(&sk_old);
    let extracted_key = sk_new.extract_key::<LWE_Params_after_extraction<MySchema>>();
    let ksk = sk_old.create_keyswitching_key::<LWE_Params_after_extraction<MySchema>>(&extracted_key);
    let eval_key = EvaluatingKey::new(bsk, ksk);
    let f = |v: u64| {
        if v==1 {1} else {0}
        // v
    };
    c.bench_function("eval", |b| b.iter(|| {
        let _: GLWECiphertext<MySchema, LWE_Params<MySchema>> = eval_key.eval(&encrypted_message, &f);
    }));
}

criterion_group!(benches, bench_bootstrap);
criterion_main!(benches);
