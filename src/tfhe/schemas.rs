#![allow(non_camel_case_types)]

use std::marker::PhantomData;

use crate::math::polynomial::polynomial::Polynomial;
use crate::random::random::{
    rnd_u64_gausean, rnd_u64_uniform, rnd_u64_uniform_binary, rnd_u64_uniform_bounded,
};
use std::fmt::{Debug};
use std::ops::{Index, IndexMut};

pub trait TFHESchema: Clone
where
    Self::ScalarType: Clone,
    Self::ScalarType: Sized,
    Self::ScalarType: from_u64,
    Self::ScalarType: Copy,

    Self::GLWECTContainerType: Debug,
    Self::GLWECTContainerType: Clone,
    Self::GLWECTContainerType: serde::ser::Serialize,
    Self::GLWECTContainerType: Sized,
    Self::GLWECTContainerType: serde::de::DeserializeOwned,
    Self::GLWECTContainerType: from_poly_list,
    // Self::GLWECTContainerType: from_poly_list<1>,
    // Self::GLWECTContainerType: from_poly_list<32>,
    // Self::GLWECTContainerType: from_poly_list<1024>,
    Self::GLWECTContainerType: Index<usize, Output = Self::ScalarType>,
    Self::GLWECTContainerType: IndexMut<usize, Output = Self::ScalarType>,

    Self::SecretKeyContainerType: Clone,
    Self::SecretKeyContainerType: Debug,
    Self::SecretKeyContainerType: from_u64_vector,
    Self::SecretKeyContainerType: serde::ser::Serialize,
    Self::SecretKeyContainerType: Sized,
    Self::SecretKeyContainerType: serde::de::DeserializeOwned,
    Self::SecretKeyContainerType: from_poly_list,
    // Self::SecretKeyContainerType: from_poly_list<1>,
    // Self::SecretKeyContainerType: from_poly_list<32>,
    // Self::SecretKeyContainerType: from_poly_list<1024>,
    Self::SecretKeyContainerType: Index<usize, Output = Self::ScalarType>,

    Self::PolynomialContainerType: Clone,
    Self::PolynomialContainerType: serde::ser::Serialize,
    Self::PolynomialContainerType: Sized,
    Self::PolynomialContainerType: serde::de::DeserializeOwned,
{
    const LWE_K: usize;
    const GLWE_N: usize;
    const GLWE_K: usize;
    const GLWE_Q: usize;
    const GLEV_B: usize;
    const GLEV_L: usize;
    const CT_MODULUS: u64;
    type ScalarType;
    type GLWECTContainerType;
    type SecretKeyContainerType;
    type PolynomialContainerType;
}

#[derive(Debug, PartialEq, Clone)]
pub struct TFHE_test_small_u64;

impl TFHESchema for TFHE_test_small_u64 {
    const LWE_K: usize = 500;
    const GLWE_N: usize = 256;
    const GLWE_K: usize = 1;
    const CT_MODULUS: u64 = u64::MAX;
    const GLWE_Q: usize = 64;
    const GLEV_B: usize = 2;
    const GLEV_L: usize = 10;
    type ScalarType = u64;
    type GLWECTContainerType = Vec<Self::ScalarType>;
    type SecretKeyContainerType = Vec<Self::ScalarType>;
    type PolynomialContainerType = Vec<Self::ScalarType>;
}

#[derive(Debug, PartialEq, Clone)]
pub struct TFHE_test_medium_u64;

impl TFHESchema for TFHE_test_medium_u64 {
    const LWE_K: usize = 586;
    const GLWE_N: usize = 1024;
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

pub trait LWE_CT_Params<S: TFHESchema>: Clone
where
    Self::ContainerType: Clone,
    Self::ContainerType: Debug,
    Self::ContainerType: serde::ser::Serialize,
    Self::ContainerType: Sized,
    Self::ContainerType: serde::de::DeserializeOwned,
    Self::ContainerType: Index<usize, Output = Self::ScalarType>,
    Self::ContainerType: IndexMut<usize, Output = Self::ScalarType>,
    Self::ContainerType: from_poly_list,

    Self::ScalarType: Clone,
    Self::ScalarType: Sized,
    Self::ScalarType: from_u64,
    Self::ScalarType: Copy,

    Self::SecretKeyContainerType: Debug,
    Self::SecretKeyContainerType: Clone,
    Self::SecretKeyContainerType: from_u64_vector,
    Self::SecretKeyContainerType: serde::ser::Serialize,
    Self::SecretKeyContainerType: Sized,
    Self::SecretKeyContainerType: serde::de::DeserializeOwned,
    Self::SecretKeyContainerType: from_poly_list,

    Self::SecretKeyContainerType: Index<usize, Output = Self::ScalarType>,
    Self::HelperType: Sized,
{
    const MASK_SIZE: usize;
    const POLINOMIAL_SIZE: usize;
    type ScalarType;
    type ContainerType;
    type SecretKeyContainerType;
    type Schema: TFHESchema;
    type HelperType= [(); Self::POLINOMIAL_SIZE] where [(); Self::POLINOMIAL_SIZE]:Sized;
    fn random_scalar_mask() -> Self::ScalarType;
    fn random_scalar_noise() -> Self::ScalarType;
    fn random_scalar_key() -> Self::ScalarType;
}
#[derive(Debug, PartialEq, Clone)]
pub struct LWE_Params<S: TFHESchema> {
    phantom: PhantomData<S>,
}

impl<S: TFHESchema> LWE_CT_Params<S> for LWE_Params<S> {
    const MASK_SIZE: usize = S::LWE_K;
    const POLINOMIAL_SIZE: usize = 1;
    type ScalarType = S::ScalarType;
    type ContainerType = S::GLWECTContainerType;
    type SecretKeyContainerType = S::SecretKeyContainerType;
    type Schema = S;
    type HelperType = [(); Self::POLINOMIAL_SIZE] where [(); Self::POLINOMIAL_SIZE]:Sized;
    fn random_scalar_mask() -> Self::ScalarType {
        // from_u64::from(rnd_u64_uniform_bounded(1 << 53))
        from_u64::from(rnd_u64_uniform())
        // from_u64::from(18_446_744_073_709_551_615u64)
        // from_u64::from(5804407862833930667)
    }
    fn random_scalar_noise() -> Self::ScalarType {
        from_u64::from(rnd_u64_gausean())
        // from_u64::from(0)
    }
    fn random_scalar_key() -> Self::ScalarType {
        from_u64::from(rnd_u64_uniform_binary())
        // from_u64::from(1)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct GLWE_Params<S: TFHESchema> {
    phantom: PhantomData<S>,
}

impl<S: TFHESchema> LWE_CT_Params<S> for GLWE_Params<S> {
    const MASK_SIZE: usize = S::GLWE_K;
    const POLINOMIAL_SIZE: usize = S::GLWE_N;
    type ScalarType = S::ScalarType;
    type ContainerType = S::GLWECTContainerType;
    type SecretKeyContainerType = S::SecretKeyContainerType;
    type Schema = S;
    type HelperType = [(); Self::POLINOMIAL_SIZE] where [(); Self::POLINOMIAL_SIZE]:Sized;
    fn random_scalar_mask() -> Self::ScalarType {
        from_u64::from(rnd_u64_uniform())
        //    from_u64::from(rnd_u64_uniform_bounded(1<<30))
        // from_u64::from(0)
    }
    fn random_scalar_noise() -> Self::ScalarType {
        from_u64::from(rnd_u64_gausean())
        // from_u64::from(0)
        // from_u64::from(rnd_u64_uniform_binary())
    }

    fn random_scalar_key() -> Self::ScalarType {
        // from_u64::from(rnd_u64_uniform_binary())
        from_u64::from(1)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct LWE_Params_after_extraction<S: TFHESchema> {
    phantom: PhantomData<S>,
}

impl<S: TFHESchema> LWE_CT_Params<S> for LWE_Params_after_extraction<S> {
    const MASK_SIZE: usize = S::GLWE_K * S::GLWE_N;
    const POLINOMIAL_SIZE: usize = 1;
    type ScalarType = S::ScalarType;
    type ContainerType = S::GLWECTContainerType;
    type SecretKeyContainerType = S::SecretKeyContainerType;
    type Schema = S;
    type HelperType = [(); Self::POLINOMIAL_SIZE] where [(); Self::POLINOMIAL_SIZE]:Sized;
    fn random_scalar_mask() -> Self::ScalarType {
        from_u64::from(rnd_u64_uniform())
    }
    fn random_scalar_noise() -> Self::ScalarType {
        from_u64::from(rnd_u64_gausean())
        // from_u64::from(0)
    }
    fn random_scalar_key() -> Self::ScalarType {
        from_u64::from(rnd_u64_uniform_binary())
    }
}

// #[derive(Debug, PartialEq, serde::ser::Serialize, Clone)]
// struct Vec_u64(Vec<u64>);

pub trait from_poly_list {
    fn from<const Order: usize>(d: Vec<Polynomial<Order>>) -> Self;
}

impl from_poly_list for Vec<u64> {
    fn from<const Order: usize>(d: Vec<Polynomial<Order>>) -> Self {
        let a = d.iter().flatten().collect::<Vec<u64>>();
        a
    }
}

pub trait from_u64_vector {
    fn from(d: Vec<u64>) -> Self;
    fn to(d: Self) -> Vec<u64>;
}

impl from_u64_vector for Vec<u64> {
    fn from(d: Vec<u64>) -> Self {
        d
    }
    fn to(d: Self) -> Vec<u64> {
        d
    }
}

pub trait from_u64 {
    fn from(d: u64) -> Self;
    fn to(d: Self) -> u64;
}

impl from_u64 for u64 {
    fn from(d: u64) -> Self {
        d
    }
    fn to(d: Self) -> u64 {
        d
    }
}

// impl Into<Vec<u64>> for Vec<Polynomial<32>> {

//   fn into(d: Vec<Polynomial<32>>) -> Vec<Polynomial<32>> {
//     Vec::new()
//   }
// }
