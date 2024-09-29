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
use clap::{Parser, Subcommand};
use colored::Colorize;

#[derive(Debug, Subcommand)]
enum AppCommand {
    Encrypt {
        operand1: u8,
        operand2: u8
    },
    Multiply,
    Decrypt,

}

#[derive(Parser)]
struct CliArgs {
    #[clap(subcommand)]
    command: AppCommand,
    #[clap(short, long, value_parser(clap::value_parser!(bool)), default_value_t=false)]
    verbose: bool,
}

fn main() {

    let args @ CliArgs { verbose, .. } = CliArgs::parse();

    let print_verbose = |s: String| {
        if verbose {
            println!("{}", s);
        }
    };

    match args {
        CliArgs {
            command: AppCommand::Encrypt {operand1, operand2},
            ..
        } => {
            
            // assert_eq!(str_to_be_encrypted.len(), str_to_compare.len(), "{}", format!("String lengths must match").red());
            // assert!(str_to_be_encrypted.chars().chain(str_to_compare.chars()).all(|x| x.is_ascii_lowercase()), "{}", format!("String should contain only latin letters in lower case").red() );

            let key: SecretKey<MySchema, LWE_Params<MySchema>> = SecretKey::new();

            print_verbose(format!("{} {:?}", format!("Secret key:").green(), &key));

            print_verbose(format!("{} {:?}", format!("Plain operand 1:").green(), &operand1));
            let encrypted_operand1 = key.encrypt_int(operand1 as u64);
            print_verbose(format!("{} {:?}", format!("Encrypted operand 1:").green(), &encrypted_operand1));

            print_verbose(format!("{} {:?}", format!("Plain operand 2:").green(), &operand2));
            let encrypted_operand2 = key.encrypt_int(operand2 as u64);
            print_verbose(format!("{} {:?}", format!("Encrypted operand 2:").green(), &encrypted_operand2));

            let eval_key = key.make_eval_key();

            let encrypted_result = eval_key.multiply(&encrypted_operand1, &encrypted_operand2);
            print_verbose(format!("{} {:?}", format!("Encrypted multiplication result:").green(), &encrypted_result));

            let result = key.decrypt_int(&encrypted_result);
            print_verbose(format!("{} {:?}", format!("Decrypted multiplication result:").green(), &result));
            println!("Received result of multiplication of encrypted number {operand1} and encrypted number {operand2}: {}, expected result: {}", result, operand1*operand2);
           
            println!("{}", format!("Done").green());
        },
        Multiply => {
            
        },
        Decrypt => {
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
struct MySchema;

impl TFHESchema for MySchema {
    const MESSAGE_SPACE_SIZE: usize = 6;
    const LWE_K: usize = 500;
    const GLWE_N: usize = 2048;
    const GLWE_K: usize = 1;
    const CT_MODULUS: u64 = u64::MAX;
    const GLWE_Q: usize = 64;
    const GLEV_B: usize = 10;
    const GLEV_L: usize = 2;
    type ScalarType = u64;
    type GLWECTContainerType = Vec<Self::ScalarType>;
    type SecretKeyContainerType = Vec<Self::ScalarType>;
    type PolynomialContainerType = Vec<Self::ScalarType>;
}

#[derive(Debug)]
struct IntCt<S: TFHESchema, P: LWE_CT_Params<S>>(GLWECiphertext<S, P>);



#[derive(Debug)]
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


    pub fn encrypt_int(&self, c: u64) -> IntCt<S, P> {
        IntCt(
            self.0
                .encrypt(&Polynomial::<{ P::POLINOMIAL_SIZE }>::new_monomial(
                    c << (S::GLWE_Q - S::GLEV_B),
                    0,
                )),
        )
    }

    #[allow(dead_code)]
    pub fn decrypt_int(&self, ct: &IntCt<S, P>) -> u64 {
        let m = self.0.decrypt(&ct.0);

        m[0] >> (S::GLWE_Q - S::GLEV_B)
    }

    pub fn make_eval_key(&self) -> EvalKey<S, P, GLWE_Params<S>> {
        let sk_new: GLWE_secret_key<S, GLWE_Params<S>> =
            GLWE_secret_key::<S, GLWE_Params<S>>::new_random();
        let extracted_key = sk_new.extract_key::<LWE_Params_after_extraction<S>>();
        let ksk = self
            .0
            .create_keyswitching_key::<LWE_Params_after_extraction<S>>(&extracted_key);
        let bsk: BootstrappingKey<S, P, GLWE_Params<S>> = sk_new.create_bootstrapping_key(&self.0);
        let eval_key = EvaluatingKey::new(bsk, ksk);
        EvalKey(eval_key)
    }
}

struct EvalKey<S: TFHESchema, PLwe: LWE_CT_Params<S>, PGlwe: LWE_CT_Params<S>>(
    EvaluatingKey<S, PLwe, PGlwe>,
);
impl<S: TFHESchema, PLwe: LWE_CT_Params<S>, PGlwe: LWE_CT_Params<S>> EvalKey<S, PLwe, PGlwe>
where
    [(); PLwe::POLINOMIAL_SIZE]: Sized,
    [(); PGlwe::POLINOMIAL_SIZE]: Sized,
    [(); LWE_Params_after_extraction::<S>::POLINOMIAL_SIZE]: Sized,
    [(); S::GLWE_Q]: Sized,
    [(); S::GLEV_B]: Sized,
    [(); S::GLEV_L]: Sized,
{
    pub fn multiply(&self, lhs: &IntCt<S, PLwe>, rhs: &IntCt<S, PLwe>) -> IntCt<S, PLwe> {
        let shift = Polynomial::<{ PLwe::POLINOMIAL_SIZE }>::new_monomial(8, 0);
        let shifted_rhs = &rhs.0 * &shift; 

        let f = |combined_operand:u64| {
            let rhs = combined_operand & 7;
            let lhs = (combined_operand & (7<<3)) >> 3;
            rhs*lhs
        };

        let sum = &lhs.0 + &shifted_rhs;
        let is_eq = self.0.eval(&sum, &f);
        IntCt(is_eq)
    }

}
