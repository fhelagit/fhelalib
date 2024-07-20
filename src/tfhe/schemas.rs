#![allow(non_camel_case_types)]

use std::marker::PhantomData;

use crate::math::polynomial::polynomial::Polynomial;
use std::fmt::{self, Debug, Display};
use std::ops::Index;

pub trait TFHESchema
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

    Self::SecretKeyContainerType: Clone,
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

#[derive(Debug, PartialEq)]
pub struct TFHE_test_small_u64;

impl TFHESchema for TFHE_test_small_u64 {
    const LWE_K: usize = 1;
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

#[derive(Debug, PartialEq)]
pub struct TFHE_test_medium_u64;

impl TFHESchema for TFHE_test_medium_u64 {
    const LWE_K: usize = 586;
    const GLWE_N: usize = 1024;
    const GLWE_K: usize = 1;
    const CT_MODULUS: u64 = u64::MAX;
    const GLWE_Q: usize = 64;
    const GLEV_B: usize = 8;
    const GLEV_L: usize = 3;
    type ScalarType = u64;
    type GLWECTContainerType = Vec<Self::ScalarType>;
    type SecretKeyContainerType = Vec<Self::ScalarType>;
    type PolynomialContainerType = Vec<Self::ScalarType>;
}

pub trait LWE_CT_Params<S: TFHESchema>
where
    Self::ContainerType: Clone,
    Self::ContainerType: Debug,
    Self::ContainerType: serde::ser::Serialize,
    Self::ContainerType: Sized,
    Self::ContainerType: serde::de::DeserializeOwned,
    Self::ContainerType: Index<usize, Output = Self::ScalarType>,
    Self::ContainerType: from_poly_list,

    Self::ScalarType: Clone,
    Self::ScalarType: Sized,
    Self::ScalarType: from_u64,
    Self::ScalarType: Copy,

    Self::SecretKeyContainerType: Clone,
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
}
#[derive(Debug, PartialEq)]
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
}

#[derive(Debug, PartialEq)]
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
}

// #[derive(Debug, PartialEq, serde::ser::Serialize, Clone)]
// struct Vec_u64(Vec<u64>);

pub trait from_poly_list {
    fn from<const Order: usize>(d: Vec<Polynomial<Order>>) -> Self;
}

impl from_poly_list for Vec<u64> {
    fn from<const Order: usize>(d: Vec<Polynomial<Order>>) -> Self {
        //  let a = Vec::with_capacity(d.len()*Order);
        let a = d.iter().flatten().collect::<Vec<u64>>();
        dbg!(a)
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
