#![allow(non_camel_case_types)]


use std::marker::PhantomData;

use crate::math::polynomial::polynomial::Polynomial;
use std::ops::Index;



pub trait TFHESchema
where 
  Self::GLWECTContainerType: Clone,
  Self::GLWECTContainerType: serde::ser::Serialize,
  Self::GLWECTContainerType: Sized,
  Self::GLWECTContainerType: serde::de::DeserializeOwned,
  Self::GLWECTContainerType: from_poly_list<1>,
  Self::GLWECTContainerType: from_poly_list<32>,
  Self::GLWECTContainerType: from_poly_list<1024>,

  Self::SecretKeyContainerType: Clone,
  Self::SecretKeyContainerType: serde::ser::Serialize,
  Self::SecretKeyContainerType: Sized,
  Self::SecretKeyContainerType: serde::de::DeserializeOwned,
  Self::SecretKeyContainerType: from_poly_list<1>,
  Self::SecretKeyContainerType: from_poly_list<32>,
  Self::SecretKeyContainerType: from_poly_list<1024>,
  Self::SecretKeyContainerType: Index<usize>,

  Self::PolynomialContainerType: Clone,
  Self::PolynomialContainerType: serde::ser::Serialize,
  Self::PolynomialContainerType: Sized,
  Self::PolynomialContainerType: serde::de::DeserializeOwned,
  {
    const LWE_K: usize;
    const GLWE_N: usize;
    const GLWE_K: usize;
    const CT_MODULUS: u64;
    type ScalarType;
    type GLWECTContainerType;
    type SecretKeyContainerType;
    type PolynomialContainerType;
}

#[derive(Debug, PartialEq)]
pub struct TFHE_test_small_u64;

impl TFHESchema for TFHE_test_small_u64 {
    const LWE_K: usize = 2;
    const GLWE_N: usize = 32;
    const GLWE_K: usize = 1;
    const CT_MODULUS: u64 = u64::MAX;
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
  type ScalarType = u64;
  type GLWECTContainerType = Vec<Self::ScalarType>;
  type SecretKeyContainerType = Vec<Self::ScalarType>;
  type PolynomialContainerType = Vec<Self::ScalarType>;
}


pub trait LWE_CT_Params 
where
  Self::ContainerType: Clone,
  Self::ContainerType: serde::ser::Serialize,
  Self::ContainerType: Sized,
  Self::ContainerType: serde::de::DeserializeOwned
{
  const MASK_SIZE: usize;
  const POLINOMIAL_SIZE: usize;
  type ScalarType;
  type ContainerType;
}
#[derive(Debug, PartialEq)]
pub struct LWE_Params<S: TFHESchema>{phantom: PhantomData<S>}

impl<S: TFHESchema>  LWE_CT_Params for LWE_Params<S> {
  const MASK_SIZE: usize = S::LWE_K;
  const POLINOMIAL_SIZE: usize= 1;
  type ScalarType = S::ScalarType;
  type ContainerType = S::GLWECTContainerType;
}

#[derive(Debug, PartialEq)]
pub struct GLWE_Params<S: TFHESchema>{phantom: PhantomData<S>}

impl<S: TFHESchema>  LWE_CT_Params for GLWE_Params<S> {
  const MASK_SIZE: usize = S::GLWE_K;
  const POLINOMIAL_SIZE: usize= S::GLWE_N;
  type ScalarType = S::ScalarType;
  type ContainerType = S::GLWECTContainerType;
}

// #[derive(Debug, PartialEq, serde::ser::Serialize, Clone)]
// struct Vec_u64(Vec<u64>);

pub trait from_poly_list<const Order: usize> {

  fn from(d: Vec<Polynomial<Order>>) -> Self;  
}

impl<const Order: usize> from_poly_list<Order> for Vec<u64> {

  fn from(d: Vec<Polynomial<Order>>) -> Self {
  //  let a = Vec::with_capacity(d.len()*Order);
    let a = d.iter().flatten().collect::<Vec<u64>>();
    a
  } 
}

// impl Into<Vec<u64>> for Vec<Polynomial<32>> {

//   fn into(d: Vec<Polynomial<32>>) -> Vec<Polynomial<32>> {
//     Vec::new()
//   } 
// }