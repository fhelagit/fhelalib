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

use crate::math::modular::module_switch::*;
use crate::math::polynomial::ct_ntt::*;

// use std::marker::PhantomData;
const Q: usize = 18446744073709550593-1;//18437455399478099969-1;//u64::MAX as usize;//18446744073709550593-1;//18446744073709550593-1;// 18446744073709547521 - 1 ;//18446744073709551521 - 1 ;//u64::MAX as usize -100;
// const Q: usize = u64::MAX as usize;
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct Polynomial<const ORDER: usize>(Vec<u64>);

impl<const ORDER: usize> Polynomial<ORDER> {
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
    #[allow(dead_code)]
    pub fn new_monomial(value: u64, position: usize) -> Self {
        let mut p = Polynomial::<ORDER>::new_zero();
        if position >= ORDER {
            p[position % ORDER] = value;
        } else {
            p[position] = value;
        }
        p
    }

    fn coeffs(&self) -> Vec<u64> {
        self.0.clone()
    }

    pub fn new_zero() -> Self {
        let mut d: Vec<u64> = Vec::with_capacity(ORDER);
        for _ in 0..ORDER {
            d.push(0);
        }
        Polynomial::new(d)
    }

    pub fn shr(&self, steps: usize) -> Self {
        Polynomial::new(self.0.iter().map(|v| v >> steps).collect())
    }
    pub fn shl(&self, steps: usize) -> Self {
        Polynomial::new(self.0.iter().map(|v| (((*v as u128) << steps) % (Q as u128 +1)) as u64).collect())
    }

    pub fn round(&self, divisor: u64) -> Self {
        Polynomial::new(self.0.iter().map(|v| (((*v as u128 + (divisor>>1) as u128) % (Q as u128 +1)) / divisor as u128) as u64).collect())
    }

    pub fn rem(&self, divisor: u64) -> Self {
        Polynomial::new(self.0.iter().map(|v| v % divisor).collect())
    }

    pub fn swicth_order<const NEW_OREDER: usize>(&self) -> Polynomial<NEW_OREDER> {
        assert_eq!(NEW_OREDER, ORDER);
        return Polynomial::<NEW_OREDER>::new(self.0.clone());
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
            "{:?}",
            self.0 //serde_json::to_string(&(*self.0).to_vec()).unwrap()
        )
        .unwrap();
        Ok(())
    }
}

impl<const ORDER: usize> FromStr for Polynomial<ORDER> {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let data: Vec<u64> = serde_json::from_str(s).unwrap();
        Ok(Polynomial::<ORDER>::new(data))
    }
}

#[cfg(test)]
#[test]
fn polynomial_str_serialization() {
    // todo make iterative, make random
    let a = [1, 2, 3, 4, 5, 6, 7, 8, 9, 0].to_vec();
    let poly: Polynomial<10> = Polynomial::new(a);

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
        let mut sums = [0; ORDER].to_vec();

        for i in 0..ORDER {
            // sums[i] = self.coeffs()[i].wrapping_add(rhs.coeffs()[i]);
            sums[i] = ((self[i] as u128 + rhs[i] as u128) % (Q as u128 + 1)) as u64;
        }
        Polynomial::new(sums)
    }
}

impl<const ORDER: usize> ops::Sub<&Polynomial<ORDER>> for &Polynomial<ORDER> {
    type Output = Polynomial<ORDER>;

    fn sub(self, rhs: &Polynomial<ORDER>) -> Polynomial<ORDER> {
        let mut diffs = [0; ORDER].to_vec();

        for i in 0..ORDER {
            // diffs[i] = self[i].wrapping_sub(rhs[i]);
            if self[i] >= rhs[i] {
                diffs[i] = ((self[i] as u128 - rhs[i] as u128) % (Q as u128 + 1)) as u64;
            } else {
                // diffs[i] = ((Q as u128 + self[i] as u128 - rhs[i] as u128) % Q as u128) as u64;
                diffs[i] = (Q as u128+1 - (rhs[i] as u128 - self[i] as u128) ) as u64;
            }
            
        }
        // println!("sub. lhs: {}, rhs: {}, diff1: {}, diff2: {}", self[0], rhs[0], rhs[0] as u128 - self[0] as u128, (Q as u128+1 - (rhs[0] as u128 - self[0] as u128) ));
        Polynomial::new(diffs)
    }
}

#[cfg(test)]
#[test]
fn test_add_polynomial() {
    // todo make iterative, make random
    const ORDER: usize = 10;
    let a = [u64::MAX, 2, 3, 4, 5, 6, 7, 8, 9, 0].to_vec();
    let b = [1, 2, 3, 4, 5, 6, 7, 8, 9, 0].to_vec();
    let sum = a
        .iter()
        .zip(b.iter())
        .map(|(ai, bi)| ai.wrapping_add(*bi))
        .collect::<Vec<u64>>();

    let poly_a: Polynomial<ORDER> = Polynomial::new(a);
    let poly_b: Polynomial<ORDER> = Polynomial::new(b);
    let poly_sum = &poly_a + &poly_b;

    assert_eq!(sum, *poly_sum.coeffs());
}

#[cfg(test)]
proptest! {
    #![proptest_config(ProptestConfig::with_cases(1000))]
    #[test]
    fn pt_add_polynomial_1000(poly_a in any::<Polynomial::<1000>>(), poly_b in any::<Polynomial::<1000>>()) {

        let a: Vec<u64> = poly_a.coeffs();
        let b: Vec<u64> = poly_b.coeffs();
        let sum = a
            .iter()
            .zip(b.iter())
            .map(|(ai, bi)| ai.wrapping_add(*bi))
            .collect::<Vec<u64>>();
        let poly_sum = &poly_a + &poly_b;
        assert_eq!(sum, *poly_sum.coeffs());
    }

    #[test]
    fn pt_add_polynomial_1(poly_a in any::<Polynomial::<1>>(), poly_b in any::<Polynomial::<1>>()) {

        let a: Vec<u64> = poly_a.coeffs();
        let b: Vec<u64> = poly_b.coeffs();
        let sum = a
            .iter()
            .zip(b.iter())
            .map(|(ai, bi)| ai.wrapping_add(*bi))
            .collect::<Vec<u64>>();
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
    fn pt_sub_polynomial_1000(poly_a in any::<[u64; 1000]>().prop_map(|v| Polynomial::<1000>::new(v.to_vec()))
    , poly_b in any::<[u64; 1000]>().prop_map(|v| Polynomial::<1000>::new(v.to_vec()))) {
        let a: Vec<u64> = poly_a.coeffs();
        let b: Vec<u64> = poly_b.coeffs();
        let sum = a
            .iter()
            .zip(b.iter())
            .map(|(ai, bi)| ai.wrapping_sub(*bi))
            .collect::<Vec<u64>>();
        let poly_sum = &poly_a - &poly_b;
        assert_eq!(sum, *poly_sum.coeffs());
    }

    #[test]
    fn pt_sub_polynomial_1(poly_a in any::<Polynomial::<1>>(), poly_b in any::<Polynomial::<1>>()) {
        let a: Vec<u64> = poly_a.coeffs();
        let b: Vec<u64> = poly_b.coeffs();
        let sum = a
            .iter()
            .zip(b.iter())
            .map(|(ai, bi)| ai.wrapping_sub(*bi))
            .collect::<Vec<u64>>();
        let poly_sum = &poly_a - &poly_b;
        assert_eq!(sum, *poly_sum.coeffs());
    }

}

impl<const ORDER: usize> ops::Mul<&Polynomial<ORDER>> for &Polynomial<ORDER> {
    type Output = Polynomial<ORDER>;

    fn mul(self, rhs: &Polynomial<ORDER>) -> Polynomial<ORDER> {
        if ORDER == 1 {
            // return Polynomial::new_monomial(self[0].wrapping_mul(rhs[0]), 0);
            return Polynomial::new_monomial(((self[0] as u128 * rhs[0] as u128) % (Q as u128 + 1)) as u64, 0);
        }
        // polymul_pwc_naive(self, rhs)
        polymul_pwc(self, rhs)
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
#[allow(dead_code)]
const nwc_n: usize = 32;

#[allow(dead_code)]
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

    Polynomial::new(d)
}

#[allow(dead_code)]
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

    let mut a_: Vec<u64> = a.coeffs();
    let mut b_: Vec<u64> = b.coeffs();

    for i in 0..n {
        a_[i] = (((a[i] as u128 * pow(psi, i as u32, q) as u128) % q as u128) % q as u128) as u64;
        b_[i] = (((b[i] as u128 * pow(psi, i as u32, q) as u128) % q as u128) % q as u128) as u64;
    }

    let mut a_ntt_form: Vec<u64> = [0; n].to_vec();
    let mut b_ntt_form: Vec<u64> = [0; n].to_vec();
    ct_ntt(&mut a_, n, q, w, &mut a_ntt_form).unwrap();
    ct_ntt(&mut b_, n, q, w, &mut b_ntt_form).unwrap();

    let mut c_ntt_form: Vec<u64> = [0; n].to_vec();
    let mut c_regular_form: Vec<u64> = [0; n].to_vec();
    let mut c_: Vec<u64> = [0; n].to_vec();

    for i in 0..n {
        c_ntt_form[i] = ((a_ntt_form[i] as u128 * b_ntt_form[i] as u128) % q as u128) as u64;
    }

    ct_intt(&mut c_ntt_form, n, q, w_inv, n_inv, &mut c_regular_form).unwrap();

    for i in 0..n {
        c_[i] = (((c_regular_form[i] as u128 * pow(psi_inv, i as u32, q) as u128) % q as u128)
            % q as u128) as u64
    }

    let c: Vec<u64> = c_
        .iter()
        .map(|v| mod_switch(*v, q as u128, 1 << 64))
        .collect();
    Polynomial::new(c)
}

#[cfg(test)]
proptest! {
    #![proptest_config(ProptestConfig::with_cases(1000))]
    #[test]
    fn pt_polymul_naive_expected(a_ in any::<[u16; nwc_n]>().prop_map(|v| Polynomial::<nwc_n>::new(v.iter().map(|x| *x as u64).collect::<Vec<u64>>()))) {
        let a = Polynomial::<pwc_n>::new_monomial(5, 0); //a_.clone();
        let b = Polynomial::<pwc_n>::new_monomial(1, 0);

        let c_naive = polymul_pwc_naive(&a, &b);
        prop_assert_eq!(dbg!(Polynomial::new(a.into_iter().map(|v| v*1).collect())), dbg!(c_naive));
        // assert_eq!(1,2)

    }
}

#[cfg(test)]
proptest! {
    #![proptest_config(ProptestConfig::with_cases(1000))]
    #[test]
    fn pt_polymul_nwc_naive_comparation(a_ in any::<[u16; nwc_n]>().prop_map(|v| Polynomial::<nwc_n>::new(v.iter().map(|x| *x as u64).collect::<Vec<u64>>()))
                                      , b_ in any::<[u16; nwc_n]>().prop_map(|v| Polynomial::<nwc_n>::new(v.iter().map(|x| *x as u64).collect::<Vec<u64>>()))) {
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
    fn pt_polymul_nwc_neutral_element(a_ in any::<[u64; nwc_n]>().prop_map(|v| Polynomial::<nwc_n>::new(v.to_vec()))) {

        let a = a_.clone();
        let mut b = Polynomial::<nwc_n>::new_zero();
        b[0] = 1;


        let c = polymul_nwc(&a, &b);
        prop_assert_eq!(c, a)

    }
}

#[cfg(test)]
proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]
    #[test]
    fn pt_polymul_nwc_absorbent_element(a_ in any::<[u64; nwc_n]>().prop_map(|v| Polynomial::<nwc_n>::new(v.to_vec()))) {
        let a = a_.clone();
        let b = Polynomial::<nwc_n>::new_zero();

        let c = polymul_nwc(&a, &b);
        prop_assert_eq!(c, b)

    }
}

#[cfg(test)]
proptest! {
    #![proptest_config(ProptestConfig::with_cases(1000))]
    #[test]
    fn pt_polymul_nwc_commutative(a_ in any::<[u64; nwc_n]>().prop_map(|v| Polynomial::<nwc_n>::new(v.to_vec()))
                                , b_ in any::<[u64; nwc_n]>().prop_map(|v| Polynomial::<nwc_n>::new(v.to_vec()))) {
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
    fn pt_polymul_nwc_accociative(a_ in any::<[u64; nwc_n]>().prop_map(|v| Polynomial::<nwc_n>::new(v.to_vec()))
                                , b_ in any::<[u64; nwc_n]>().prop_map(|v| Polynomial::<nwc_n>::new(v.to_vec()))
                                , c_ in any::<[u64; nwc_n]>().prop_map(|v| Polynomial::<nwc_n>::new(v.to_vec()))) {
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
    fn pt_polymul_nwc_distributive(   a_ in any::<[u64; nwc_n]>().prop_map(|v| Polynomial::<nwc_n>::new(v.to_vec()))
                                    , b_ in any::<[u64; nwc_n]>().prop_map(|v| Polynomial::<nwc_n>::new(v.to_vec()))
                                    , c_ in any::<[u64; nwc_n]>().prop_map(|v| Polynomial::<nwc_n>::new(v.to_vec()))) {
        // let q: u64       = 18446744073709547521;
        let a = a_.clone();
        let b = b_.clone();
        let c = c_.clone();


        let a_plus_b = &a + &b;// = polymul(&a, &b);
        let a_plus_b_mul_c = polymul_nwc(&a_plus_b, &c); //:Vec<u64> = [0; n].to_vec();

        let a_mul_c = polymul_nwc(&a, &c);
        let b_mul_c = polymul_nwc(&b, &c);
        let a_mul_c_plus_b_mul_c = &a_mul_c + &b_mul_c;


        prop_assert_eq!(a_plus_b_mul_c, a_mul_c_plus_b_mul_c)

    }
}

// PWC
#[allow(dead_code)]
const pwc_n: usize = 32;

#[allow(dead_code)]
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

    // 256
    let q: u64 = 18446744073709550593;
    let w: u64 = 12400524647368804660;
    let w_inv: u64 = 14137232041405300922;
    const n: usize = 256;
    let n_inv: u64 = 18374686479671622661;


    // 32
    // let q: u64 = 18446744073709551521;
    // let w: u64 = 2250779155537587393;
    // let w_inv: u64 = 18006900733222636570;
    // const n: usize = 32;
    // let n_inv: u64 = 17870283321406128036;

    // mod switch

    // let mut a_: Vec<u64> = a
    //     .coeffs()
    //     .iter()
    //     .map(|v| mod_switch(*v, 1 << 64, q as u128))
    //     .collect();
    // let mut b_: Vec<u64> = b
    //     .coeffs()
    //     .iter()
    //     .map(|v| mod_switch(*v, 1 << 64, q as u128))
    //     .collect();

    let mut a_ntt_form: Vec<u64> = [0; n].to_vec();
    let mut b_ntt_form: Vec<u64> = [0; n].to_vec();
    ct_ntt(&mut a.coeffs(), n, q, w, &mut a_ntt_form).unwrap();
    ct_ntt(&mut b.coeffs(), n, q, w, &mut b_ntt_form).unwrap();

    let mut c_ntt_form: Vec<u64> = [0; n].to_vec();
    let mut c_regular_form: Vec<u64> = [0; n].to_vec();

    for i in 0..n {
        c_ntt_form[i] = ((a_ntt_form[i] as u128 * b_ntt_form[i] as u128) % q as u128) as u64;
    }

    ct_intt(&mut c_ntt_form, n, q, w_inv, n_inv, &mut c_regular_form).unwrap();

    // mod switch back

    // let c: Vec<u64> = c_regular_form
    //     .iter()
    //     .map(|v| mod_switch(*v, q as u128, 1 << 64))
    //     .collect();
    Polynomial::new(c_regular_form)
}

fn polymul_pwc_naive<const ORDER: usize>(
    a: &Polynomial<ORDER>,
    b: &Polynomial<ORDER>,
) -> Polynomial<ORDER> {
    let mut c: Vec<u64> = Vec::with_capacity(2 * ORDER);
    for _ in 0..2 * ORDER {
        c.push(0);
    }

    for i in 0..ORDER {
        for j in 0..ORDER {
            // c[i + j] = c[i + j].wrapping_add(a[i].wrapping_mul(b[j]));
            c[i + j] = (c[i + j] as u128 + ((a[i] as u128 * b[j] as u128) % (Q as u128 + 1)) % (Q as u128 + 1)) as u64;
        }
    }

    let mut d: Vec<u64> = Vec::with_capacity(ORDER);
    for i in 0..ORDER {
        // d.push(c[i].wrapping_add(c[i + ORDER]));
        d.push(
            ((c[i] as u128 + c[i + ORDER] as u128) % (Q as u128 + 1)) as u64
        );
    }

    Polynomial::new(d)
}

#[cfg(test)]
proptest! {
    #![proptest_config(ProptestConfig::with_cases(1000))]
    #[test]
    fn pt_polymul_pwc_naive_comparation(a_ in any::<[u16; pwc_n]>().prop_map(|v| Polynomial::<pwc_n>::new(v.iter().map(|x| *x as u64).collect::<Vec<u64>>()))
    , b_ in any::<[u16; pwc_n]>().prop_map(|v| Polynomial::<pwc_n>::new(v.iter().map(|x| *x as u64).collect::<Vec<u64>>()))) {

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
    fn pt_polymul_pwc_neutral_element(a_ in any::<[u64; pwc_n]>().prop_map(|v| Polynomial::<pwc_n>::new(v.to_vec()))) {
        // так как единица при модуль-свитчинге превращается в 0, то на выходе мы всегда получаем ноль.
        // Поэтому пока допустимая точность равна всему оступному диапазону чисел.


        let a = a_.clone();
        let mut b = Polynomial::<pwc_n>::new_zero();
        b[0] = 1;


        let c = polymul_pwc(&a, &b);
        prop_assert_eq!(poly_approximately_equial::<pwc_n>(&c, &a, u64::MAX), true)

    }
}

#[cfg(test)]
proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]
    #[test]
    fn pt_polymul_pwc_absorbent_element(a_ in any::<[u64; pwc_n]>().prop_map(|v| Polynomial::<pwc_n>::new(v.to_vec()))) {

        let a = a_.clone();
        let b = Polynomial::<pwc_n>::new_zero();

        let c = polymul_pwc(&a, &b);
        prop_assert_eq!(c, b)

    }
}

#[cfg(test)]
proptest! {
    #![proptest_config(ProptestConfig::with_cases(1000))]
    #[test]
    fn pt_polymul_pwc_commutative(a_ in any::<[u64; pwc_n]>().prop_map(|v| Polynomial::<pwc_n>::new(v.to_vec()))
                                , b_ in any::<[u64; pwc_n]>().prop_map(|v| Polynomial::<pwc_n>::new(v.to_vec()))) {
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
    fn pt_polymul_pwc_associative(a_ in any::<[u64; pwc_n]>().prop_map(|v| Polynomial::<pwc_n>::new(v.to_vec()))
                                , b_ in any::<[u64; pwc_n]>().prop_map(|v| Polynomial::<pwc_n>::new(v.to_vec()))
                                , c_ in any::<[u64; pwc_n]>().prop_map(|v| Polynomial::<pwc_n>::new(v.to_vec()))) {
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
    fn pt_polymul_pwc_distributive(a_ in any::<[u64; pwc_n]>().prop_map(|v| Polynomial::<pwc_n>::new(v.to_vec()))
        , b_ in any::<[u64; pwc_n]>().prop_map(|v| Polynomial::<pwc_n>::new(v.to_vec()))
        , c_ in any::<[u64; pwc_n]>().prop_map(|v| Polynomial::<pwc_n>::new(v.to_vec()))) {
        // кажется здесь нужно более изощренная проверка по примерное равенство

        let a = a_.clone();
        let b = b_.clone();
        let c = c_.clone();


        let a_plus_b:Polynomial<pwc_n> = &a + &b;// = polymul(&a, &b);

        let a_plus_b_mul_c = polymul_pwc(&a_plus_b, &c); //:Vec<u64> = [0; n].to_vec();

        let a_mul_c = polymul_pwc(&a, &c);
        let b_mul_c = polymul_pwc(&b, &c);
        let a_mul_c_plus_b_mul_c = &a_mul_c + &b_mul_c;


        prop_assert_eq!(poly_approximately_equial::<pwc_n>(&a_plus_b_mul_c, &a_mul_c_plus_b_mul_c, 1000000000), true)

    }
}

impl<const ORDER: usize> IntoIterator for &Polynomial<ORDER> {
    type Item = u64;

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.to_vec().into_iter()
    }
}

// pub fn decompose_polynomial<const ORDER: usize>(p: Polynomial<ORDER>) -> Vec<Polynomial<ORDER>> {

// }

pub fn decompose_polynomial<
    const GLWE_Q: usize,
    const GLEV_L: usize,
    const GLEV_B: usize,
    const ORDER: usize,
>(
    p: Polynomial<ORDER>,
) -> Vec<Polynomial<ORDER>> {
    let mut a = Vec::with_capacity(GLEV_L);
    for _ in 0..GLEV_L {
        a.push([0; ORDER].to_vec());
    }
    //let b:[Vec<u64>; S::GLEV_L] = a.try_into().unwrap();
    // println!("nums: {:?}", p.coeffs());
    let decs = p
        .coeffs()
        .iter()
        .map(|x| {
            let dec = decomp_int::<{ GLWE_Q }, { GLEV_L }, { GLEV_B }>(*x);
            // println!("dec_int({}) = {:?}", x, dec);
            dec
        })
        .into_iter()
        .enumerate()
        .fold(a, |acc, (i, dec_nums)| {
            let acc_ = acc
                .iter()
                .enumerate()
                .map(|(j, ns)| {
                    let mut ns_ = ns.clone();
                    // println!("ns_: {:?}", ns_);
                    // println!("j: {:?}", j);
                    ns_[i] = dec_nums[j];
                    ns_
                })
                .collect::<Vec<Vec<u64>>>();
            acc_
            // заменить в каждом j-том полиномеме i-тый компонент на j-тый компонент dec_nums
        })
        .iter()
        .map(|e| Polynomial::new(e.clone()))
        .collect();
    decs
}

fn decomp_int<const GLWE_Q: usize, const GLEV_L: usize, const GLEV_B: usize>(n: u64) -> Vec<u64> {
    let pos = (GLEV_L * GLEV_B) as u32;

    

    let bit = if pos == 64 {
        0
    } else {
        n & (1 << (GLWE_Q as u32 - pos - 1))
    };

    let new_n = if bit > 0 && pos < 64 {
        n // n.wrapping_add(1 << (GLWE_Q as u32 - pos))
    } else {
        n
    };

    let res = (0..GLEV_L)
        .into_iter()
        .map(|i| {
            let l_shift = new_n << (GLEV_B * i);
            let r_shift = (l_shift) >> (GLWE_Q - GLEV_B);
            r_shift
        })
        .collect::<Vec<u64>>();
    res
}

#[cfg(test)]
proptest! {
    #![proptest_config(ProptestConfig::with_cases(1000))]
    #[test]
    fn pt_decomp_int_callable(a in any::<u64>()) {

        let _ = decomp_int::<64, 3, 8>(a);


        // prop_assert_eq!(poly_approximately_equial::<pwc_n>(&d_ab_c, &d_a_bc, 10000), true)

    }
}

#[cfg(test)]
proptest! {
    #![proptest_config(ProptestConfig::with_cases(1000))]
    #[test]
    fn pt_decomp_poly_callable(a in any::<[u64; pwc_n]>().prop_map(|v| Polynomial::<pwc_n>::new(v.to_vec()))) {

        let _ = decompose_polynomial::<64, 3, 8, pwc_n>(a);


        // prop_assert_eq!(poly_approximately_equial::<pwc_n>(&d_ab_c, &d_a_bc, 10000), true)


    }
}
