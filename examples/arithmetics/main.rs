#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

use clap::{Parser, Subcommand};
use colored::Colorize;
use std::fs::File;
use std::io::{Read, Write};
use tfhela::{
    math::polynomial::polynomial::Polynomial,
    tfhe::{
        glwe::GLWECiphertext,
        schemas::{
            from_u64, from_u64_vector, GLWE_Params, LWE_CT_Params, LWE_Params,
            LWE_Params_after_extraction, TFHESchema,
        },
        secret_key::secret_key::GLWE_secret_key,
        server_key::server_key::{BootstrappingKey, EvaluatingKey},
    },
};

const o1_fn: &str = "o1.ct";
const o2_fn: &str = "o2.ct";
const result_fn: &str = "result.ct";
const key_fn: &str = "key.sk";
const eval_key_fn: &str = "eval_key.ek";

#[derive(Debug, Subcommand)]
enum AppCommand {
    Multiply { operand1: u8, operand2: u8 },
    //Multiply,
    Decrypt { filename: String },
    MakeSecretKey,
    MakeEvalKey,
    Encrypt1 { operand: u8},
    Encrypt2 { operand: u8},
    Mult
}

#[derive(Parser)]
struct CliArgs {
    #[clap(subcommand)]
    command: AppCommand,
    // #[clap(short, long, value_parser(clap::value_parser!(bool)), default_value_t=false)]
    // verbose: bool,
}

fn main() {
    let args @ CliArgs { .. } = CliArgs::parse();

    let print_verbose = |s: String| {
        // if verbose {
        println!("{}", s);
        // }
    };

    match args {
        CliArgs {
            command: AppCommand::Multiply { operand1, operand2 },
            ..
        } => {
            assert!(operand1 >= 0 && operand1<=7, "{}", format!("Both operands should be not negative integer less then 8").red());
            assert!(operand2 >= 0 && operand2<=7, "{}", format!("Both operands should be not negative integer less then 8").red());
            // assert!(str_to_be_encrypted.chars().chain(str_to_compare.chars()).all(|x| x.is_ascii_lowercase()), "{}", format!("String should contain only latin letters in lower case").red() );

            let key: SecretKey<MySchema, LWE_Params<MySchema>> = SecretKey::new();
            save_key(key_fn, &key);
            let key: SecretKey<MySchema, LWE_Params<MySchema>> = load_key(key_fn).unwrap();

            print_verbose(format!(
                "{} {}",
                format!("Secret key: ").green(),
                "********"
            ));

            print_verbose(format!(
                "{} {:?}",
                format!("Plain operand 1:").green(),
                &operand1
            ));
            let encrypted_operand1 = key.encrypt_int(operand1 as u64);
            print_verbose(format!(
                "{} {:?}",
                format!("Encrypted operand 1:").green(),
                encrypted_operand1.show()
            ));
            save_ct(&o1_fn, &encrypted_operand1);

            print_verbose(format!(
                "{} {:?}",
                format!("Plain operand 2:").green(),
                &operand2
            ));
            let encrypted_operand2 = key.encrypt_int(operand2 as u64);
            print_verbose(format!(
                "{} {:?}",
                format!("Encrypted operand 2:").green(),
                encrypted_operand2.show()
            ));

            save_ct(o2_fn, &encrypted_operand2);
            let encrypted_operand2 = load_ct(o2_fn).unwrap();

            let eval_key = key.make_eval_key();

            let encrypted_result = eval_key.multiply(&encrypted_operand1, &encrypted_operand2);
            print_verbose(format!(
                "{} {:?}",
                format!("Encrypted multiplication result:").green(),
                encrypted_result.show()
            ));
            save_ct(result_fn, &encrypted_result);

            let result = key.decrypt_int(&encrypted_result);
            print_verbose(format!(
                "{} {:?}",
                format!("Decrypted multiplication result:").green(),
                &result
            ));
            let expected: u64 = (operand1 * operand2) as u64;
            let same = result == expected;
            let pick_color = |str, same| {
                if same {
                    format!("{}", str).green()
                } else {
                    format!("{}", str).red()
                }
            };
            println!("Received result of multiplication of encrypted number {operand1} and encrypted number {operand2}: {}, expected result: {}", 
                pick_color(result, same),
                pick_color(expected, same));

            println!("{}", format!("Secret key is stored in file: {key_fn}"));
            println!(
                "{}",
                format!("Encrypted operand 1 is stored in file: {o1_fn}")
            );
            println!(
                "{}",
                format!("Encrypted operand 2 is stored in file: {o2_fn}")
            );
            println!(
                "{}",
                format!("Encrypted result is stored in file: {result_fn}")
            );
        }
        // CliArgs {
        //     command: AppCommand::Multiply,
        //     ..
        // } => {

        // },
        CliArgs {
            command: AppCommand::Decrypt { filename },
            ..
        } => {
            let encrypted: IntCt<MySchema, LWE_Params<MySchema>> =
                load_ct(filename.as_str()).unwrap();
            let key: SecretKey<MySchema, LWE_Params<MySchema>> = load_key(key_fn).unwrap();
            let decrypted = key.decrypt_int(&encrypted);
            println!("Decrypt ciphertext stored in \"{filename}\" using secret key stored in \"{key_fn}\"");
            println!("Corresponding plain value is: {}", decrypted);
        }
        CliArgs {
            command: AppCommand::Encrypt1 { operand },
            ..
        } => {
            assert!(operand >= 0 && operand<=7, "{}", format!("Both operands should be not negative integer less then 8").red());
            let key: SecretKey<MySchema, LWE_Params<MySchema>> = load_key(key_fn).unwrap();
            print_verbose(format!("Load secret key from file: {} ...", key_fn));
            let encrypted_operand = key.encrypt_int(operand as u64);
            print_verbose(format!(
                "{} {:?}",
                format!("Encrypted operand 1:").green(),
                encrypted_operand.show()
            ));
            print_verbose(format!(
                "{} {}",
                format!("Encrypted operand 1 stored in file:").green(),
                o1_fn
            ));
            save_ct(&o1_fn, &encrypted_operand);
        }
        CliArgs {
            command: AppCommand::Encrypt2 { operand },
            ..
        } => {
            assert!(operand >= 0 && operand<=7, "{}", format!("Both operands should be not negative integer less then 8").red());
            let key: SecretKey<MySchema, LWE_Params<MySchema>> = load_key(key_fn).unwrap();
            print_verbose(format!("Load secret key from file: {} ...", key_fn));
            let encrypted_operand = key.encrypt_int(operand as u64);
            print_verbose(format!(
                "{} {:?}",
                format!("Encrypted operand 2:").green(),
                encrypted_operand.show()
            ));
            print_verbose(format!(
                "{} {}",
                format!("Encrypted operand 2 stored in file:").green(),
                o2_fn
            ));
            save_ct(&o2_fn, &encrypted_operand);
        }
        CliArgs {
            command: AppCommand::Mult,
            ..
        } => {
            let key: SecretKey<MySchema, LWE_Params<MySchema>> = load_key(key_fn).unwrap();
            let eval_key = key.make_eval_key();
            let encrypted_operand1: IntCt<MySchema, LWE_Params<MySchema>> =
                load_ct(o1_fn).unwrap();
            let encrypted_operand2: IntCt<MySchema, LWE_Params<MySchema>> =
                load_ct(o2_fn).unwrap();
            let encrypted_result = eval_key.multiply(&encrypted_operand1, &encrypted_operand2);
            print_verbose(format!(
                "{} {:?}",
                format!("Encrypted multiplication result:").green(),
                encrypted_result.show()
            ));
            save_ct(result_fn, &encrypted_result);
        }
        CliArgs {
            command: AppCommand::MakeSecretKey,
            ..
        } => {
            let key: SecretKey<MySchema, LWE_Params<MySchema>> = SecretKey::new();
            print_verbose(format!("Load secret key from file: {} ...", key_fn));
            print_verbose(format!("Create evaluation key  ..."));
            save_key(key_fn, &key);
            print_verbose(format!(
                "{} {}",
                format!("Secret key: ").green(),
                "********"
            ));
            print_verbose(format!(
                "{} {}",
                format!("Secret key stored in file:").green(),
                key_fn
            ));
        }
        CliArgs {
            command: AppCommand::MakeEvalKey,
            ..
        } => {
            let key: SecretKey<MySchema, LWE_Params<MySchema>> = SecretKey::new();
            print_verbose(format!("Load secret key from file: {} ...", key_fn));
            print_verbose(format!("Create evaluation key  ..."));
            let key: SecretKey<MySchema, LWE_Params<MySchema>> = load_key(key_fn).unwrap();
            let eval_key = key.make_eval_key();
            save_key(key_fn, &key);

            print_verbose(format!(
                "{} {}",
                format!("Secret key stored in file:").green(),
                key_fn
            ));
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
    const GLEV_L: usize = 3;
    type ScalarType = u64;
    type GLWECTContainerType = Vec<Self::ScalarType>;
    type SecretKeyContainerType = Vec<Self::ScalarType>;
    type PolynomialContainerType = Vec<Self::ScalarType>;
}

#[derive(Debug)]
struct IntCt<S: TFHESchema, P: LWE_CT_Params<S>>(GLWECiphertext<S, P>);
impl<S: TFHESchema, P: LWE_CT_Params<S>> IntCt<S, P> {
    fn show(&self) -> String {
        format!(
            "[{:?}, {:?}, {:?}, ... {:?} bytes ..., {:?}, {:?}, {:?}]",
            from_u64::to(self.0[0]),
            from_u64::to(self.0[1]),
            from_u64::to(self.0[2]),
            ((P::MASK_SIZE + 1) * P::POLINOMIAL_SIZE - 6) * 8,
            from_u64::to(self.0[(P::MASK_SIZE + 1) * P::POLINOMIAL_SIZE - 3]),
            from_u64::to(self.0[(P::MASK_SIZE + 1) * P::POLINOMIAL_SIZE - 2]),
            from_u64::to(self.0[(P::MASK_SIZE + 1) * P::POLINOMIAL_SIZE - 1])
        )
    }
}

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

        let f = |combined_operand: u64| {
            let rhs = combined_operand & 7;
            let lhs = (combined_operand & (7 << 3)) >> 3;
            rhs * lhs
        };

        let sum = &lhs.0 + &shifted_rhs;
        let is_eq = self.0.eval(&sum, &f);
        IntCt(is_eq)
    }
}

fn save_ct<S: TFHESchema, P: LWE_CT_Params<S>>(
    filename: &str,
    ct: &IntCt<S, P>,
) -> std::io::Result<()>
where
    [(); (P::MASK_SIZE + 1) * P::POLINOMIAL_SIZE]: Sized,
{
    let mut file = File::create(filename)?;
    for i in 0..(P::MASK_SIZE + 1) * P::POLINOMIAL_SIZE {
        file.write_all(&from_u64::to(ct.0[i]).to_le_bytes())
            .unwrap();
    }
    Ok(())
}

pub fn load_ct<S: TFHESchema, P: LWE_CT_Params<S>>(filename: &str) -> std::io::Result<IntCt<S, P>>
where
    [(); (P::MASK_SIZE + 1) * P::POLINOMIAL_SIZE * 8]: Sized,
{
    let mut file = File::open(filename)?;
    let mut content_ = [0 as u8; (P::MASK_SIZE + 1) * P::POLINOMIAL_SIZE * 8];
    file.read_exact(&mut content_)?;
    let mut content: Vec<u64> = Vec::with_capacity(content_.len() / 8);
    for i in 0..content_.len() / 8 {
        content.push(u64::from_le_bytes(
            content_[i * 8..i * 8 + 8].try_into().unwrap(),
        ));
        // content.push(u64::from_le_bytes([108,247,11,137,61,199,171,150].try_into().unwrap()));
    }
    Ok(IntCt(GLWECiphertext::from_polynomial_list(
        from_u64_vector::from(content),
    )))
}

pub fn save_key<S: TFHESchema, P: LWE_CT_Params<S>>(
    filename: &str,
    key: &SecretKey<S, P>,
) -> std::io::Result<()>
where
    [(); (P::MASK_SIZE + 1) * P::POLINOMIAL_SIZE]: Sized,
{
    let mut file = File::create(filename)?;
    for i in 0..P::MASK_SIZE * P::POLINOMIAL_SIZE {
        file.write_all(&from_u64::to(key.0.to_u64_vector()[i]).to_le_bytes())
            .unwrap();
    }
    Ok(())
}

// pub fn save_eval_key<S: TFHESchema, PLwe: LWE_CT_Params<S>, PGlwe: LWE_CT_Params<S>>(
//     filename: &str,
//     key: &EvalKey<S, PLwe, PGlwe>,
// ) -> std::io::Result<()>
// where
//     [(); PLwe::MASK_SIZE
//         * (PGlwe::MASK_SIZE + 1) * PGlwe::POLINOMIAL_SIZE 
//         * PGlwe::GLEV_L
//         * (PGlwe::MASK_SIZE + 1) 
//      + (PLwe::MASK_SIZE + 1) * PLwe::GLEV_L * (PGlwe::MASK_SIZE) * PLwe::POLINOMIAL_SIZE ]: Sized,
// {
//     let mut file = File::create(filename)?;
//     for i in 0..P::MASK_SIZE * P::POLINOMIAL_SIZE {
//         file.write_all(&from_u64::to(key.0.to_u64_vector()[i]).to_le_bytes())
//             .unwrap();
//     }
//     Ok(())
// }

pub fn load_key<S: TFHESchema, P: LWE_CT_Params<S>>(
    filename: &str,
) -> std::io::Result<SecretKey<S, P>>
where
    [(); P::MASK_SIZE * P::POLINOMIAL_SIZE * 8]: Sized,
{
    let mut file = File::open(filename)?;
    let mut content_ = [0 as u8; P::MASK_SIZE * P::POLINOMIAL_SIZE * 8];
    file.read_exact(&mut content_)?;
    let mut content: Vec<u64> = Vec::with_capacity(content_.len() / 8);
    for i in 0..content_.len() / 8 {
        content.push(u64::from_le_bytes(
            content_[i * 8..i * 8 + 8].try_into().unwrap(),
        ));
        // content.push(u64::from_le_bytes([108,247,11,137,61,199,171,150].try_into().unwrap()));
    }

    Ok(SecretKey(GLWE_secret_key::from_scalar_vector(
        from_u64_vector::from(content),
    )))
}
