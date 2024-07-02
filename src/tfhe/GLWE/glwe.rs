pub struct GLWECiphertext(Box<[u64]>);

impl GLWECiphertext{
  pub fn new(data: Box<[u64]>) -> Self {
    GLWECiphertext(data)
  }
}
