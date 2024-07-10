use std::ops;
use std::ops::{Index, IndexMut};

use std::fmt::{self, Display};
use std::str::FromStr;
extern crate serde_json;

#[cfg(test)]
use proptest::prelude::*;
#[cfg(test)]
use proptest_derive::Arbitrary;

use crate::math::modular::mod_arith::*;
use crate::math::modular::module_switch::*;
use crate::math::polynomial::ct_ntt::*;

#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct Polynomial<const ORDER: usize>(Box<[u64; ORDER]>);

impl<const ORDER: usize> Polynomial<ORDER> {
    fn new(data: Box<[u64; ORDER]>) -> Self {
        Polynomial(data)
    }

    fn new_monomial(value: u64, position: usize) -> Self {
        let mut d = [0; ORDER];
        d[position] = value;
        Polynomial(Box::new(d))
    }

    fn coeffs(&self) -> Box<[u64; ORDER]> {
        self.0.clone()
    }
}

impl<const ORDER: usize> Index<usize> for Polynomial<ORDER> {
    type Output = u64;
    fn index(&self, i: usize) -> &u64 {
        &self.0[i]
    }
}

impl<const ORDER: usize> IndexMut<usize> for Polynomial<ORDER> {
    fn index_mut(&mut self, i: usize) -> &mut u64 {
        &mut self.0[i]
    }
}

// Serialization

impl<const ORDER: usize> Display for &Polynomial<ORDER> {
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
        Ok(Polynomial::<ORDER>::new(Box::new(data.try_into().unwrap())))
    }
}

#[cfg(test)]
#[test]
fn polynomial_str_serialization() {
    // todo make iterative, make random
    let a = [1, 2, 3, 4, 5, 6, 7, 8, 9, 0];
    let poly: Polynomial<10> = Polynomial::new(Box::new(a));

    let serialized = (&poly).to_string();
    let deserialized: Polynomial<10> = FromStr::from_str(&serialized).unwrap();
    assert_eq!(poly, deserialized);
}

#[cfg(test)]
proptest! {
    #![proptest_config(ProptestConfig::with_cases(1000))]
    #[test]
    fn pt_polynomial_str_serialization_100(poly in any::<Polynomial::<100>>()) {
        println!("{}", &poly);
        let serialized = (&poly).to_string();
        let deserialized: Polynomial<100> = FromStr::from_str(&serialized).unwrap();
        prop_assert_eq!(poly, deserialized);

    }
}

#[cfg(test)]
proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]
    #[test]
    fn pt_polynomial_str_serialization_1000(poly in any::<Polynomial::<1000>>()) {
        println!("{}", &poly);
        let serialized = (&poly).to_string();
        let deserialized: Polynomial<1000> = FromStr::from_str(&serialized).unwrap();
        prop_assert_eq!(poly, deserialized);

    }
}

// ops
impl<const ORDER: usize> ops::Add<&Polynomial<ORDER>> for &Polynomial<ORDER> {
    type Output = Polynomial<ORDER>;

    fn add(self, rhs: &Polynomial<ORDER>) -> Polynomial<ORDER> {
        let mut sums = Box::new([0; ORDER]);

        for i in 0..ORDER {
            sums[i] = self.coeffs()[i].wrapping_add(rhs.coeffs()[i]);
        }
        Polynomial::new(sums)
    }
}

impl<const ORDER: usize> ops::Sub<&Polynomial<ORDER>> for &Polynomial<ORDER> {
    type Output = Polynomial<ORDER>;

    fn sub(self, rhs: &Polynomial<ORDER>) -> Polynomial<ORDER> {
        let mut sums = Box::new([0; ORDER]);

        for i in 0..ORDER {
            sums[i] = self.coeffs()[i].wrapping_sub(rhs.coeffs()[i]);
        }
        Polynomial::new(sums)
    }
}

#[cfg(test)]
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

#[cfg(test)]
proptest! {
    #![proptest_config(ProptestConfig::with_cases(1000))]
    #[test]
    fn pt_add_polynomial_1000(poly_a in any::<Polynomial::<1000>>(), poly_b in any::<Polynomial::<1000>>()) {
        const ORDER: usize = 1000;
        let a: [u64; ORDER] = *poly_a.coeffs();
        let b: [u64; ORDER] = *poly_b.coeffs();
        let sum: [u64; ORDER] = a
            .iter()
            .zip(b.iter())
            .map(|(ai, bi)| ai.wrapping_add(*bi))
            .collect::<Vec<u64>>()
            .try_into()
            .unwrap();
        let poly_sum = &poly_a + &poly_b;
        assert_eq!(sum, *poly_sum.coeffs());
    }

    #[test]
    fn pt_add_polynomial_1(poly_a in any::<Polynomial::<1>>(), poly_b in any::<Polynomial::<1>>()) {
        const ORDER: usize = 1;
        let a: [u64; ORDER] = *poly_a.coeffs();
        let b: [u64; ORDER] = *poly_b.coeffs();
        let sum: [u64; ORDER] = a
            .iter()
            .zip(b.iter())
            .map(|(ai, bi)| ai.wrapping_add(*bi))
            .collect::<Vec<u64>>()
            .try_into()
            .unwrap();
        let poly_sum = &poly_a + &poly_b;
        assert_eq!(sum, *poly_sum.coeffs());
    }

    #[test]
    fn pt_add_polynomial_commutative_1000(poly_a in any::<Polynomial::<1000>>(), poly_b in any::<Polynomial::<1000>>()) {
        assert_eq!(&poly_a + &poly_b, &poly_b + &poly_a);
    }
}

#[cfg(test)]
proptest! {
    #![proptest_config(ProptestConfig::with_cases(1000))]
    #[test]
    fn pt_sub_polynomial_1000(poly_a in any::<Polynomial::<1000>>(), poly_b in any::<Polynomial::<1000>>()) {
        const ORDER: usize = 1000;
        let a: [u64; ORDER] = *poly_a.coeffs();
        let b: [u64; ORDER] = *poly_b.coeffs();
        let sum: [u64; ORDER] = a
            .iter()
            .zip(b.iter())
            .map(|(ai, bi)| ai.wrapping_sub(*bi))
            .collect::<Vec<u64>>()
            .try_into()
            .unwrap();
        let poly_sum = &poly_a - &poly_b;
        assert_eq!(sum, *poly_sum.coeffs());
    }

    #[test]
    fn pt_sub_polynomial_1(poly_a in any::<Polynomial::<1>>(), poly_b in any::<Polynomial::<1>>()) {
        const ORDER: usize = 1;
        let a: [u64; ORDER] = *poly_a.coeffs();
        let b: [u64; ORDER] = *poly_b.coeffs();
        let sum: [u64; ORDER] = a
            .iter()
            .zip(b.iter())
            .map(|(ai, bi)| ai.wrapping_sub(*bi))
            .collect::<Vec<u64>>()
            .try_into()
            .unwrap();
        let poly_sum = &poly_a - &poly_b;
        assert_eq!(sum, *poly_sum.coeffs());
    }

}

impl<const ORDER: usize> ops::Mul<&Polynomial<ORDER>> for &Polynomial<ORDER> {
    type Output = Polynomial<ORDER>;

    fn mul(self, rhs: &Polynomial<ORDER>) -> Polynomial<ORDER> {
        polymul_pwc_naive(self, rhs)
    }
}

fn poly_approximately_equial<const ORDER: usize>(
    a: &Polynomial<ORDER>,
    b: &Polynomial<ORDER>,
    delta: u64,
) -> bool {
    let mut res = true;
    for i in 0..ORDER {
        if b[i]
            < if a[i].wrapping_sub(delta) > a[i] {
                0
            } else {
                a[i].wrapping_sub(delta)
            }
            || b[i]
                > if a[i].wrapping_add(delta) < a[i] {
                    u64::MAX
                } else {
                    a[i].wrapping_add(delta)
                }
        {
            println!(
                "poly_approximately_equial(), coefficients aren't equial: a[{}] = {}, b[{}] = {}",
                i, a[i], i, b[i],
            );
            res = false;
            break;
        }
    }
    res
}

// NWC
const nwc_n: usize = 2048;

fn polymul_nwc_naive<const ORDER: usize>(
    a: &Polynomial<ORDER>,
    b: &Polynomial<ORDER>,
) -> Polynomial<ORDER> {
    let mut c: Vec<u64> = [0; 2 * nwc_n].to_vec();
    let mut d: Vec<u64> = [0; nwc_n].to_vec();

    for i in 0..nwc_n {
        for j in 0..nwc_n {
            c[i + j] = &c[i + j] + &(&(a[i]) * &(b[j]));
        }
    }

    for i in 0..nwc_n {
        d[i] = &c[i] - &c[i + nwc_n];
    }

    Polynomial::new(Box::new(d.try_into().unwrap()))
}

fn polymul_nwc<const ORDER: usize>(
    a: &Polynomial<ORDER>,
    b: &Polynomial<ORDER>,
) -> Polynomial<ORDER> {
    // 2048
    let q: u64 = 18446744073709547521;
    let w: u64 = 13871691955188213127;
    let w_inv: u64 = 7236465593496852055;
    const n: usize = 2048;
    let n_inv: u64 = 18437736874454806531;
    let psi: u64 = 3618691915695908984;
    let psi_inv: u64 = 6610778516587902706;

    // 32
    // let q: u64       = 18446744073709550593;
    // let w: u64       = 13709748631181643000;
    // let w_inv: u64   = 15941171536453849061;
    // const n: usize   = 32;
    // let n_inv: u64   = 17870283321406127137;
    // let psi: u64     = 9059689486234189519;
    // let psi_inv: u64 = 13125114981792952;

    // let mut a_: Vec<u64> = [0; n].to_vec();
    // let mut b_: Vec<u64> = [0; n].to_vec();

    let mut a_: Vec<u64> = a.coeffs().to_vec();
    let mut b_: Vec<u64> = b.coeffs().to_vec();

    for i in 0..n {
        a_[i] = (((a[i] as u128 * pow(psi, i as u32, q) as u128) % q as u128) % q as u128) as u64;
        b_[i] = (((b[i] as u128 * pow(psi, i as u32, q) as u128) % q as u128) % q as u128) as u64;
    }

    let mut a_ntt_form: Vec<u64> = [0; n].to_vec();
    let mut b_ntt_form: Vec<u64> = [0; n].to_vec();
    CT_ntt(&mut a_, n, q, w, &mut a_ntt_form).unwrap();
    CT_ntt(&mut b_, n, q, w, &mut b_ntt_form).unwrap();

    let mut c_ntt_form: Vec<u64> = [0; n].to_vec();
    let mut c_regular_form: Vec<u64> = [0; n].to_vec();
    let mut c_: Vec<u64> = [0; n].to_vec();

    for i in 0..n {
        c_ntt_form[i] = ((a_ntt_form[i] as u128 * b_ntt_form[i] as u128) % q as u128) as u64;
    }

    CT_intt(&mut c_ntt_form, n, q, w_inv, n_inv, &mut c_regular_form).unwrap();

    for i in 0..n {
        c_[i] = (((c_regular_form[i] as u128 * pow(psi_inv, i as u32, q) as u128) % q as u128)
            % q as u128) as u64
    }

    let c: Vec<u64> = c_.iter().map(|v| mod_switch(*v, q, u64::MAX)).collect();
    Polynomial::new(Box::new(c.try_into().unwrap()))
}

#[cfg(test)]
proptest! {
    #![proptest_config(ProptestConfig::with_cases(1000))]
    #[test]
    fn pt_polymul_nwc_naive_comparation(a_ in any::<[u16; nwc_n]>().prop_map(|v| Polynomial::new(Box::new(v.iter().map(|x| *x as u64).collect::<Vec<u64>>().try_into().unwrap())))
                                      , b_ in any::<[u16; nwc_n]>().prop_map(|v| Polynomial::new(Box::new(v.iter().map(|x| *x as u64).collect::<Vec<u64>>().try_into().unwrap())))) {
        let a = a_.clone();
        let b = b_.clone();

        let c_nnt = polymul_nwc(&a, &b);
        let c_naive = polymul_nwc_naive(&a, &b);
        prop_assert_eq!(poly_approximately_equial::<nwc_n>(&c_nnt, &c_naive, 1000), true)

    }
}

#[cfg(test)]
proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]
    #[test]
    fn pt_polymul_nwc_neutral_element(a_ in any::<[u64; nwc_n]>().prop_map(|v| Polynomial::new(Box::new(v)))) {

        const n: usize = nwc_n;
        let a = a_.clone();
        let mut b = Polynomial::new(Box::new([0; n]));
        b[0] = 1;


        let c = polymul_nwc(&a, &b);
        prop_assert_eq!(c, a)

    }
}

#[cfg(test)]
proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]
    #[test]
    fn pt_polymul_nwc_absorbent_element(a_ in any::<[u64; nwc_n]>().prop_map(|v| Polynomial::new(Box::new(v)))) {
        const n: usize = nwc_n;
        let a = a_.clone();
        let b = Polynomial::new(Box::new([0; n]));

        let c = polymul_nwc(&a, &b);
        prop_assert_eq!(c, b)

    }
}

#[cfg(test)]
proptest! {
    #![proptest_config(ProptestConfig::with_cases(1000))]
    #[test]
    fn pt_polymul_nwc_commutative(a_ in any::<[u64; nwc_n]>().prop_map(|v| Polynomial::new(Box::new(v)))
                                , b_ in any::<[u64; nwc_n]>().prop_map(|v| Polynomial::new(Box::new(v)))) {
        let a = a_.clone();
        let b = b_.clone();

        let cab = polymul_nwc(&a, &b);
        let cba = polymul_nwc(&b, &a);
        prop_assert_eq!(cab, cba)

    }
}

#[cfg(test)]
proptest! {
    #![proptest_config(ProptestConfig::with_cases(1000))]
    #[test]
    fn pt_polymul_nwc_accociative(a_ in any::<[u64; nwc_n]>().prop_map(|v| Polynomial::new(Box::new(v)))
                                , b_ in any::<[u64; nwc_n]>().prop_map(|v| Polynomial::new(Box::new(v)))
                                , c_ in any::<[u64; nwc_n]>().prop_map(|v| Polynomial::new(Box::new(v)))) {
        let a = a_.clone();
        let b = b_.clone();
                let c = c_.clone();



        let d_ab_c = polymul_nwc(&polymul_nwc(&a, &b), &c);
        let d_a_bc = polymul_nwc(&a, &polymul_nwc(&b, &c));
        prop_assert_eq!(d_ab_c, d_a_bc)

    }
}

#[cfg(test)]
proptest! {
    #![proptest_config(ProptestConfig::with_cases(1000))]
    #[test]
    fn pt_polymul_nwc_distributive(   a_ in any::<[u64; nwc_n]>().prop_map(|v| Polynomial::new(Box::new(v)))
                                    , b_ in any::<[u64; nwc_n]>().prop_map(|v| Polynomial::new(Box::new(v)))
                                    , c_ in any::<[u64; nwc_n]>().prop_map(|v| Polynomial::new(Box::new(v)))) {
        const n: usize = nwc_n;
        let q: u64       = 18446744073709547521;
        let a = a_.clone();
        let b = b_.clone();
        let c = c_.clone();


        let mut a_plus_b = &a + &b;// = polymul(&a, &b);
        let a_plus_b_mul_c = polymul_nwc(&a_plus_b, &c); //:Vec<u64> = [0; n].to_vec();

        let a_mul_c = polymul_nwc(&a, &c);
        let b_mul_c = polymul_nwc(&b, &c);
        let mut a_mul_c_plus_b_mul_c = &a_mul_c + &b_mul_c;


        prop_assert_eq!(a_plus_b_mul_c, a_mul_c_plus_b_mul_c)

    }
}

// PWC
const pwc_n: usize = 32;

fn polymul_pwc<const ORDER: usize>(
    a: &Polynomial<ORDER>,
    b: &Polynomial<ORDER>,
) -> Polynomial<ORDER> {
    // 2048
    // let q: u64 = 18446744073709547521;
    // let w: u64 = 13871691955188213127;
    // let w_inv: u64 = 7236465593496852055;
    // const n: usize = 2048;
    // let n_inv: u64 = 18437736874454806531;

    // 32
    let q: u64 = 18446744073709551521;
    let w: u64 = 2250779155537587393;
    let w_inv: u64 = 18006900733222636570;
    const n: usize = 32;
    let n_inv: u64 = 17870283321406128036;

    // mod switch

    let mut a_: Vec<u64> = a
        .coeffs()
        .iter()
        .map(|v| mod_switch(*v, u64::MAX, q))
        .collect();
    let mut b_: Vec<u64> = b
        .coeffs()
        .iter()
        .map(|v| mod_switch(*v, u64::MAX, q))
        .collect();

    let mut a_ntt_form: Vec<u64> = [0; n].to_vec();
    let mut b_ntt_form: Vec<u64> = [0; n].to_vec();
    CT_ntt(&mut a_, n, q, w, &mut a_ntt_form).unwrap();
    CT_ntt(&mut b_, n, q, w, &mut b_ntt_form).unwrap();

    let mut c_ntt_form: Vec<u64> = [0; n].to_vec();
    let mut c_regular_form: Vec<u64> = [0; n].to_vec();

    for i in 0..n {
        c_ntt_form[i] = ((a_ntt_form[i] as u128 * b_ntt_form[i] as u128) % q as u128) as u64;
    }

    CT_intt(&mut c_ntt_form, n, q, w_inv, n_inv, &mut c_regular_form).unwrap();

    // mod switch back

    let c: Vec<u64> = dbg!(c_regular_form)
        .iter()
        .map(|v| mod_switch(*v, q, u64::MAX))
        .collect();
    Polynomial::new(Box::new(c.try_into().unwrap()))
}

fn polymul_pwc_naive<const ORDER: usize>(
    a: &Polynomial<ORDER>,
    b: &Polynomial<ORDER>,
) -> Polynomial<ORDER> {
    let mut c: Vec<u64> = [0; 2 * pwc_n].to_vec();
    let mut d: Vec<u64> = [0; pwc_n].to_vec();

    for i in 0..pwc_n {
        for j in 0..pwc_n {
            c[i + j] = &c[i + j] + &(&(a[i]) * &(b[j]));
        }
    }

    for i in 0..pwc_n {
        d[i] = &c[i] + &c[i + pwc_n];
    }

    Polynomial::new(Box::new(d.try_into().unwrap()))
}

#[cfg(test)]
proptest! {
    #![proptest_config(ProptestConfig::with_cases(1000))]
    #[test]
    fn pt_polymul_pwc_naive_comparation(a_ in any::<[u16; pwc_n]>().prop_map(|v| Polynomial::new(Box::new(v.iter().map(|x| *x as u64).collect::<Vec<u64>>().try_into().unwrap())))
    , b_ in any::<[u16; pwc_n]>().prop_map(|v| Polynomial::new(Box::new(v.iter().map(|x| *x as u64).collect::<Vec<u64>>().try_into().unwrap())))) {

    // так как на больших числах переход через модуль приводит к очень большому расхождению
    let a = a_.clone();
    let b = b_.clone();

    let c_nnt = polymul_pwc(&a, &b);
    let c_naive = polymul_pwc_naive(&a, &b);
    prop_assert_eq!(poly_approximately_equial::<pwc_n>(&c_nnt, &c_naive, 10000000), true)

    }
}

#[cfg(test)]
proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]
    #[test]
    fn pt_polymul_pwc_neutral_element(a_ in any::<[u64; pwc_n]>().prop_map(|v| Polynomial::new(Box::new(v)))) {
        // так как единица при модуль-свитчинге превращается в 0, то на выходе мы всегда получаем ноль.
        // Поэтому пока допустимая точность равна всему оступному диапазону чисел.

        const n: usize = pwc_n;
        let a = a_.clone();
        let mut b = Polynomial::new(Box::new([0; n]));
        b[0] = 1;


        let c = polymul_pwc(&a, &b);
        prop_assert_eq!(poly_approximately_equial::<pwc_n>(&c, &a, u64::MAX), true)

    }
}

#[cfg(test)]
proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]
    #[test]
    fn pt_polymul_pwc_absorbent_element(a_ in any::<[u64; pwc_n]>().prop_map(|v| Polynomial::new(Box::new(v)))) {
        const n: usize = pwc_n;
        let a = a_.clone();
        let b = Polynomial::new(Box::new([0; n]));

        let c = polymul_pwc(&a, &b);
        prop_assert_eq!(c, b)

    }
}

#[cfg(test)]
proptest! {
    #![proptest_config(ProptestConfig::with_cases(1000))]
    #[test]
    fn pt_polymul_pwc_commutative(a_ in any::<[u64; pwc_n]>().prop_map(|v| Polynomial::new(Box::new(v)))
                                , b_ in any::<[u64; pwc_n]>().prop_map(|v| Polynomial::new(Box::new(v)))) {
        let a = a_.clone();
        let b = b_.clone();

        let cab = polymul_pwc(&a, &b);
        let cba = polymul_pwc(&b, &a);
        prop_assert_eq!(cab, cba)

    }
}

#[cfg(test)]
proptest! {
    #![proptest_config(ProptestConfig::with_cases(1000))]
    #[test]
    fn pt_polymul_pwc_associative(a_ in any::<[u64; pwc_n]>().prop_map(|v| Polynomial::new(Box::new(v)))
                                , b_ in any::<[u64; pwc_n]>().prop_map(|v| Polynomial::new(Box::new(v)))
                                , c_ in any::<[u64; pwc_n]>().prop_map(|v| Polynomial::new(Box::new(v)))) {
        let a = a_.clone();
        let b = b_.clone();
        let c = c_.clone();

        let d_ab_c = polymul_pwc(&polymul_pwc(&a, &b), &c);
        let d_a_bc = polymul_pwc(&a, &polymul_pwc(&b, &c));
        prop_assert_eq!(poly_approximately_equial::<pwc_n>(&d_ab_c, &d_a_bc, 10000), true)

    }
}

// /////////////////////////////
#[cfg(test)]
proptest! {
    #![proptest_config(ProptestConfig::with_cases(1000))]
    #[test]
    fn pt_polymul_pwc_distributive(a_ in any::<[u64; pwc_n]>().prop_map(|v| Polynomial::new(Box::new(v)))
        , b_ in any::<[u64; pwc_n]>().prop_map(|v| Polynomial::new(Box::new(v)))
        , c_ in any::<[u64; pwc_n]>().prop_map(|v| Polynomial::new(Box::new(v)))) {
        // кажется здесь нужно более изощренная проверка по примерное равенство
        const n: usize = pwc_n;
        let a = a_.clone();
        let b = b_.clone();
        let c = c_.clone();


        let mut a_plus_b:Polynomial<pwc_n> = &a + &b;// = polymul(&a, &b);

        let a_plus_b_mul_c = polymul_pwc(&a_plus_b, &c); //:Vec<u64> = [0; n].to_vec();

        let a_mul_c = polymul_pwc(&a, &c);
        let b_mul_c = polymul_pwc(&b, &c);
        let mut a_mul_c_plus_b_mul_c = &a_mul_c + &b_mul_c;


        prop_assert_eq!(poly_approximately_equial::<pwc_n>(&a_plus_b_mul_c, &a_mul_c_plus_b_mul_c, 1000000000), true)

    }
}
