use std::fmt::{self, Display};
use std::str::FromStr;
extern crate serde_json;

use Vec;

#[derive(Debug)]
pub struct GLWECiphertext<const Polynomialsize: usize, const Masksize: usize>(Box<[u64; Polynomialsize*(Masksize+1)]>) where [(); Polynomialsize*(Masksize+1)]: Sized;

impl<const Polynomialsize: usize, const Masksize: usize> GLWECiphertext<Polynomialsize, Masksize>
where [(); Polynomialsize*(Masksize+1)]: Sized
{
  // fn new(data: Box<[u64; Polynomialsize*Masksize]>) -> Self 
  // where
  //   [(); Masksize+1]: Sized
  // {
  //   GLWECiphertext(data)
  // } 

  pub fn from_polynomial_list(data: Box<[u64; Polynomialsize*(Masksize+1)]>) -> Self {
    GLWECiphertext(data)
  }
}


impl<const Polynomialsize: usize, const Masksize: usize> Display for GLWECiphertext<Polynomialsize, Masksize> 
where [(); Polynomialsize*(Masksize+1)]: Sized {
  fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
      write!(formatter, "{}", serde_json::to_string(&(*self.0).to_vec()).unwrap()).unwrap();
      Ok(())
  }
}

impl<const Polynomialsize: usize, const Masksize: usize> FromStr for GLWECiphertext<Polynomialsize, Masksize> 
where [(); Polynomialsize*(Masksize+1)]: Sized {
  type Err = &'static str;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
      let data: Vec<u64> = serde_json::from_str(s).unwrap();
      Ok(GLWECiphertext::from_polynomial_list(Box::new(data.try_into().unwrap())))
  }
}

#[test]
fn test_glwe_to_str_serialization() {
    // todo make iterative, make random
    let a = [1,2,3,4,5,6,7,8,9,0];
    let ct: GLWECiphertext<5, 1> = GLWECiphertext::from_polynomial_list(Box::new(a));

    let serialized = ct.to_string();
    let deserialized: GLWECiphertext<5, 1> = FromStr::from_str(&serialized).unwrap();
    assert_eq!(ct.0, deserialized.0);
}
