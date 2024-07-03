pub struct Polinomial<const ORDER: usize>(Box<[u64; ORDER]>);

impl<const ORDER: usize> Polinomial<ORDER> {
    fn new(data: Box<[u64; ORDER]>) -> Self {
        Polinomial(data)
    }
}
