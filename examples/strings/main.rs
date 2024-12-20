#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

use clap::{Parser, Subcommand};
use colored::Colorize;
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

#[derive(Debug, Subcommand)]
enum AppCommand {
    CheckEquility {
        str_to_be_encrypted: String,
        str_to_compare: String,
    },
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
            command:
                AppCommand::CheckEquility {
                    str_to_be_encrypted,
                    str_to_compare,
                },
            ..
        } => {
            assert_eq!(
                str_to_be_encrypted.len(),
                str_to_compare.len(),
                "{}",
                format!("String lengths must match").red()
            );
            assert!(
                str_to_be_encrypted
                    .chars()
                    .chain(str_to_compare.chars())
                    .all(|x| x.is_ascii_lowercase()),
                "{}",
                format!("String should contain only latin letters in lower case").red()
            );

            let key: SecretKey<MySchema, LWE_Params<MySchema>> = SecretKey::new();

            print_verbose(format!("{} {:?}", format!("Secret key:").green(), &key));

            print_verbose(format!(
                "{} {:?}",
                format!("Plain string:").green(),
                &str_to_be_encrypted
            ));
            let encrypted_str = key.encrypt_string(&str_to_be_encrypted);
            print_verbose(format!(
                "{} {:?}",
                format!("Encrypted string:").green(),
                &encrypted_str
            ));

            let eval_key = key.make_eval_key();

            print_verbose(format!(
                "{} {:?}",
                format!("Plain string to compare:").green(),
                &str_to_compare
            ));
            let encrypted_result = eval_key.is_strings_eq(encrypted_str, &str_to_compare);
            print_verbose(format!(
                "{} {:?}",
                format!("Encrypted comparasion result:").green(),
                &encrypted_result
            ));

            let result = key.decrypt_bool(&encrypted_result);
            print_verbose(format!(
                "{} {:?}",
                format!("Decrypted comparasion result:").green(),
                &result
            ));
            println!("Result of checking equility of encrypted string \"{str_to_be_encrypted}\" and plain string \"{str_to_compare}\": {}", if result {"strings are same".green()} else {"string aren't same".red()});

            println!("{}", format!("Done").green());
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
struct MySchema;

impl TFHESchema for MySchema {
    const MESSAGE_SPACE_SIZE: usize = 5;
    const LWE_K: usize = 3;
    const GLWE_N: usize = 1024;
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
struct CharCt<S: TFHESchema, P: LWE_CT_Params<S>>(GLWECiphertext<S, P>);
#[derive(Debug, Clone)]
struct BoolCt<S: TFHESchema, P: LWE_CT_Params<S>>(GLWECiphertext<S, P>);

#[derive(Debug)]
struct StringCt<S: TFHESchema, P: LWE_CT_Params<S>>(Vec<CharCt<S, P>>);

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

    pub fn encrypt_string(&self, s: &String) -> StringCt<S, P> {
        let mut v: Vec<CharCt<S, P>> = Vec::with_capacity(s.len());
        for i in 0..s.len() {
            let c: u64 = s.chars().nth(i).unwrap() as u64 - 100;

            let enc_c = self.encrypt_char(c);

            v.push(enc_c);
        }
        StringCt(v)
    }

    pub fn encrypt_char(&self, c: u64) -> CharCt<S, P> {
        CharCt(
            self.0
                .encrypt(&Polynomial::<{ P::POLINOMIAL_SIZE }>::new_monomial(
                    c << (S::GLWE_Q - S::GLEV_B),
                    0,
                )),
        )
    }

    #[allow(dead_code)]
    pub fn decrypt_char(&self, ct: &CharCt<S, P>) -> u64 {
        let m = self.0.decrypt(&ct.0);

        m[0] >> (S::GLWE_Q - S::GLEV_B)
    }

    pub fn decrypt_bool(&self, ct: &BoolCt<S, P>) -> bool {
        let m = self.0.decrypt(&ct.0);

        m[0] >> (S::GLWE_Q - S::GLEV_B) == 1
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

fn encode_char(c: u64) -> u64 {
    c - 100
}

fn decode_char(c: u64) -> u64 {
    c + 100
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
    pub fn is_chars_eq(&self, ct: &CharCt<S, PLwe>, char: u64) -> BoolCt<S, PLwe> {
        let f = |v: u64| if v == encode_char(char) { 1 } else { 0 };
        let is_eq = self.0.eval(&ct.0, &f);
        BoolCt(is_eq)
    }

    pub fn is_strings_eq(&self, ct: StringCt<S, PLwe>, s: &String) -> BoolCt<S, PLwe> {
        let mut acc: BoolCt<S, PLwe> = self.is_chars_eq(&ct.0[0], s.chars().nth(0).unwrap() as u64);
        for i in 1..s.len() {
            acc = self.and(
                &acc,
                &self.is_chars_eq(&ct.0[i], s.chars().nth(i).unwrap() as u64),
            );
        }

        acc
    }

    pub fn and(&self, lhs: &BoolCt<S, PLwe>, rhs: &BoolCt<S, PLwe>) -> BoolCt<S, PLwe> {
        let shift = Polynomial::<{ PLwe::POLINOMIAL_SIZE }>::new_monomial(2, 0);
        let shifted_rhs = &rhs.0 * &shift;

        let sum = &lhs.0 + &shifted_rhs;

        let f = |v: u64| if v == 3 { 1 } else { 0 };
        let res = self.0.eval(&sum, &f);
        BoolCt(res)
    }
}
