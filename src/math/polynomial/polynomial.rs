use std::fmt::{self, Display};
use std::str::FromStr;
extern crate serde_json;

pub struct Polinomial<const ORDER: usize>(Box<[u64; ORDER]>);

impl<const ORDER: usize> Polinomial<ORDER> {
    fn new(data: Box<[u64; ORDER]>) -> Self {
        Polinomial(data)
    }
}

impl<const Polynomialsize: usize> Display for Polinomial<Polynomialsize> {
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

impl<const Polynomialsize: usize> FromStr for Polinomial<Polynomialsize> {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let data: Vec<u64> = serde_json::from_str(s).unwrap();
        Ok(Polinomial::new(Box::new(data.try_into().unwrap())))
    }
}

#[test]
fn test_polynomial_to_str_serialization() {
    // todo make iterative, make random
    let a = [1, 2, 3, 4, 5, 6, 7, 8, 9, 0];
    let ct: Polinomial<10> = Polinomial::new(Box::new(a));

    let serialized = ct.to_string();
    let deserialized: Polinomial<10> = FromStr::from_str(&serialized).unwrap();
    assert_eq!(ct.0, deserialized.0);
}
