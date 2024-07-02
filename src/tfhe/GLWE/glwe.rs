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
