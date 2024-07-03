use std::ops;

use std::fmt::{self, Display};
use std::str::FromStr;
extern crate serde_json;

#[derive(Debug, PartialEq)]
pub struct Polynomial<const ORDER: usize>(Box<[u64; ORDER]>);

impl<const ORDER: usize> Polynomial<ORDER> {
    fn new(data: Box<[u64; ORDER]>) -> Self {
        Polynomial(data)
    }

    fn coeffs(&self) -> Box<[u64; ORDER]> {
        self.0.clone()
    }
}

// Serialization

impl<const ORDER: usize> Display for Polynomial<ORDER> {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(
            formatter,
            "{}",
            serde_json::to_string(&(*self.0).to_vec()).unwrap()
        )
        .unwrap();
        Ok(())
    }
}

impl<const ORDER: usize> FromStr for Polynomial<ORDER> {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let data: Vec<u64> = serde_json::from_str(s).unwrap();
        Ok(Polynomial::new(Box::new(data.try_into().unwrap())))
    }
}

#[test]
fn test_polynomial_to_str_serialization() {
    // todo make iterative, make random
    let a = [1, 2, 3, 4, 5, 6, 7, 8, 9, 0];
    let poly: Polynomial<10> = Polynomial::new(Box::new(a));

    let serialized = poly.to_string();
    let deserialized: Polynomial<10> = FromStr::from_str(&serialized).unwrap();
    assert_eq!(poly, deserialized);
}

// ops

impl<const ORDER: usize> ops::Add<&Polynomial<ORDER>> for &Polynomial<ORDER> {
    type Output = Polynomial<ORDER>;

    fn add(self, rhs: &Polynomial<ORDER>) -> Polynomial<ORDER> {
        let mut sums = Box::new([0; ORDER]);

        for i in 0..ORDER {
            sums[i] = (self.coeffs()[i]).wrapping_add(rhs.coeffs()[i]);
        }
        Polynomial::new(sums)
    }
}

#[test]
fn test_add_polynomial() {
    // todo make iterative, make random
    const ORDER: usize = 10;
    let a: [u64; ORDER] = [u64::MAX, 2, 3, 4, 5, 6, 7, 8, 9, 0];
    let b: [u64; ORDER] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 0];
    let sum: [u64; ORDER] = a
        .iter()
        .zip(b.iter())
        .map(|(ai, bi)| ai.wrapping_add(*bi))
        .collect::<Vec<u64>>()
        .try_into()
        .unwrap();

    let poly_a: Polynomial<ORDER> = Polynomial::new(Box::new(a));
    let poly_b: Polynomial<ORDER> = Polynomial::new(Box::new(b));
    let poly_sum = &poly_a + &poly_b;

    assert_eq!(sum, *poly_sum.coeffs());
}
