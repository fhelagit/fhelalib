use tfhela::{math::polynomial::polynomial::Polynomial, tfhe::{glwe::GLWECiphertext, schemas::{GLWE_Params, LWE_Params, LWE_Params_after_extraction, TFHESchema, TFHE_test_small_u64}, secret_key::{secret_key::GLWE_secret_key}, server_key::{server_key::{BootstrappingKey, EvaluatingKey}}}};

fn main() {
    let message = Polynomial::<1>::new_monomial(1<<(TFHE_test_small_u64::GLWE_Q-TFHE_test_small_u64::GLEV_B), 0);

    let sk_old: GLWE_secret_key<TFHE_test_small_u64, LWE_Params<TFHE_test_small_u64>> =
        GLWE_secret_key::new_random();

    let sk_new: GLWE_secret_key<TFHE_test_small_u64, GLWE_Params<TFHE_test_small_u64>> =
        GLWE_secret_key::new_random();
    let extracted_key = sk_new.extract_key::<LWE_Params_after_extraction<TFHE_test_small_u64>>();
    let ksk = sk_old.create_keyswitching_key::<LWE_Params_after_extraction<TFHE_test_small_u64>>(
        &extracted_key,
    );
    let bsk: BootstrappingKey<
        TFHE_test_small_u64,
        LWE_Params<TFHE_test_small_u64>,
        GLWE_Params<TFHE_test_small_u64>,
    > = sk_new.create_bootstrapping_key(&sk_old);
    let eval_key = EvaluatingKey::new(bsk, ksk);

    let encrypted_message: GLWECiphertext<TFHE_test_small_u64, LWE_Params<TFHE_test_small_u64>> =
        sk_old.encrypt(&message);

    let f = |v: u64| v;

    let evaluated_message = eval_key.eval(&encrypted_message, &f);

    let expected_message =
        message.shr(TFHE_test_small_u64::GLWE_Q - TFHE_test_small_u64::GLEV_B)[0];

    let decrypted_message = sk_old
        .decrypt(&evaluated_message)
        .shr(TFHE_test_small_u64::GLWE_Q - TFHE_test_small_u64::GLEV_B)[0];

    assert_eq!(decrypted_message, expected_message);
    println!("Ok")
}
