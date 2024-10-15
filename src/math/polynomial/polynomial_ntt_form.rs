#![allow(dead_code)]


use std::ops;
use std::ops::{Index, IndexMut};

use std::fmt::{self, Display};
use std::str::FromStr;
extern crate serde_json;

#[cfg(test)]
use proptest::prelude::*;
#[cfg(test)]
use proptest_derive::Arbitrary;

use crate::math::modular::mod_arith::{mod_sub, mod_sum, mod_mul};
use crate::math::modular::module_switch::*;
use crate::math::polynomial::ct_ntt::*;

use super::polynomial::Polynomial;

#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct PolynomialNttForm<const ORDER: usize>(Vec<u64>);

impl<const ORDER: usize> PolynomialNttForm<ORDER> {
    pub fn new(data: Vec<u64>) -> Self {
        assert_eq!(
            ORDER,
            data.len(),
            "Attempt to create polynomial with order {} from vector with lenght {}",
            ORDER,
            data.len()
        );
        Polynomial(data)
    }
  
    fn coeffs(&self) -> Vec<u64> {
        self.0.clone()
    }

    fn from_polinomial(p: &Polynomial<ORDER>) -> Self{
        let mut a_ntt_form: Vec<u64> = a.coeffs(); 
    }

    fn to_polinomial(&self) -> Polynomial<ORDER> {

    }

}

impl<const ORDER: usize> Index<usize> for PolynomialNttForm<ORDER> {
    type Output = u64;
    fn index(&self, i: usize) -> &u64 {
        &self.0[i]
    }
}

impl<const ORDER: usize> IndexMut<usize> for PolynomialNttForm<ORDER> {
    fn index_mut(&mut self, i: usize) -> &mut u64 {
        &mut self.0[i]
    }
}

// Serialization

impl<const ORDER: usize> Display for &PolynomialNttForm<ORDER> {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(
            formatter,
            "{:?}",
            self.0 //serde_json::to_string(&(*self.0).to_vec()).unwrap()
        )
        .unwrap();
        Ok(())
    }
}

impl<const ORDER: usize> FromStr for PolynomialNttForm<ORDER> {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let data: Vec<u64> = serde_json::from_str(s).unwrap();
        Ok(PolynomialNttForm::<ORDER>::new(data))
    }
}

