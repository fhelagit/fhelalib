type ntt_data_size = u64;
use crate::{
    math::modular::mod_arith::*,
    math::modular::module_switch::*,
};
#[cfg(test)]
use proptest::prelude::*;
#[cfg(test)]
use proptest_derive::Arbitrary;

#[cfg(test)]
proptest! {
  #![proptest_config(ProptestConfig::with_cases(100))]
  #[test]
  fn pt_ntt_invertible(regular_form_ in any::<[u64; 2048]>().prop_map(|v| v.to_vec())) {
        const n: usize = 2048;
        let q: u64 = 18446744073709547521;
        let w: u64 = 13871691955188213127;
        let w_inv: u64 = 7236465593496852055;
        let n_inv: u64 = 18437736874454806531;
        let mut regular_form = regular_form_.clone();
        let mut nnt_form = [0; n].to_vec();

        CT_ntt(&mut regular_form, n,  q, w, &mut nnt_form).unwrap();
        println!("1. reg_form: {:?}, \n   ntt_form: {:?}", regular_form, nnt_form);
        CT_intt(&mut nnt_form, n,  q, w_inv, n_inv, &mut regular_form).unwrap();
        println!("2. reg_form: {:?}, \n   ntt_form: {:?}", regular_form, nnt_form);
        prop_assert_eq!(regular_form_, regular_form)




  }
}

#[test]
fn test_ntt_ones() {
    const n: usize = 32;
    let q: u64 = 18446744073709551521;
    let w: u64 = 2250779155537587393;
    let mut regular_form = [1; n].to_vec();
    let mut ntt_form = [0; n].to_vec();
    let mut expected_ntt_form = [0; n].to_vec();
    expected_ntt_form[0] = n as u64;

    CT_ntt(&mut regular_form, n, q, w, &mut ntt_form).unwrap();
    assert_eq!(ntt_form, expected_ntt_form)
}

#[test]
fn test_intt_ones() {
    const n: usize = 32;
    let q: u64 = 18446744073709551521;
    let w_inv: u64 = 18006900733222636570;
    let n_inv: u64 = 17870283321406128036;
    let mut regular_form = [0; n].to_vec();
    let mut ntt_form = [0; n].to_vec();
    ntt_form[0] = n as u64;
    let mut expected_regular_form = [1; n].to_vec();

    CT_intt(&mut ntt_form, n, q, w_inv, n_inv, &mut regular_form).unwrap();
    assert_eq!(regular_form, expected_regular_form)
}

// q = 18446744073709550593
// i = 9059689486234189519
// q: 18446744073709550593,
// w: 13709748631181643000,
// w_inv: 15941171536453849061,
// n: 32,
// n_inv: 17870283321406127137

#[cfg(test)]
proptest! {
  #![proptest_config(ProptestConfig::with_cases(100))]
  #[test]
  fn pt_nc_ntt_invertible(regular_form_ in any::<[u64; 2048]>().prop_map(|v| v.to_vec())) {
        // q: 18446744073709547521,
        // w: 13871691955188213127,
        // w_inv: 7236465593496852055,
        // n: 2048,
        // n_inv: 18437736874454806531,
        // psi: 3618691915695908984,
        // psi_inv: 6610778516587902706

        const n: usize = 2048;
        let q: u64 = 18446744073709547521;
        let w: u64 = 3618691915695908984;
        let w_inv: u64 = 6610778516587902706;
        let n_inv: u64 = 18437736874454806531;
        let mut regular_form = regular_form_.clone();
        let mut nnt_form = [0; n].to_vec();

        CT_ntt(&mut regular_form, n,  q, w, &mut nnt_form).unwrap();
        println!("1. reg_form: {:?}, \n   ntt_form: {:?}", regular_form, nnt_form);
        CT_intt(&mut nnt_form, n,  q, w_inv, n_inv, &mut regular_form).unwrap();
        println!("2. reg_form: {:?}, \n   ntt_form: {:?}", regular_form, nnt_form);
        prop_assert_eq!(regular_form_, regular_form)




  }
}
#[test]
fn test_nc_ntt_ones() {
    const n: usize = 32;
    let q: u64 = 18446744073709550593;
    let w: u64 = 13709748631181643000;
    let mut regular_form = [1; n].to_vec();
    let mut ntt_form = [0; n].to_vec();
    let mut expected_ntt_form = [0; n].to_vec();
    expected_ntt_form[0] = n as u64;

    CT_ntt(&mut regular_form, n, q, w, &mut ntt_form).unwrap();
    assert_eq!(ntt_form, expected_ntt_form)
}

#[test]
fn test_nc_intt_ones() {
    const n: usize = 32;
    let q: u64 = 18446744073709550593;
    let w_inv: u64 = 15941171536453849061;
    let n_inv: u64 = 17870283321406127137;
    let mut regular_form = [0; n].to_vec();
    let mut ntt_form = [0; n].to_vec();
    ntt_form[0] = n as u64;
    let mut expected_regular_form = [1; n].to_vec();

    CT_intt(&mut ntt_form, n, q, w_inv, n_inv, &mut regular_form).unwrap();
    assert_eq!(regular_form, expected_regular_form)
}
// todo
// умножние полиномов
// коммутативность
// ассоциативность
// диытрибутивность
// нейтральный
// поглощающий
// вращение
// маленькие
// большие
// умножение на константу
// функцию примерного старвнеия полиномов

pub fn pow(a: u64, p: u32, q: u64) -> u64 {
    let mut acc: u64 = 1;
    for _ in 1..=p {
        acc = ((acc as u128 * a as u128) % q as u128) as u64;
    }
    acc
}

pub fn egcd(a: u64, b: u64) -> (u64, u64, u64) {
    match a {
        0 => (b, 0, 1),
        a => {
            let (g, x, y) = egcd(b % a, a);
            (g, x - (b / a) * y, y)
        }
    }
}

pub fn modinv(a: u64, q: u64) -> Result<u64, String> {
    let (g, x, y) = egcd(a, q);
    match g {
        1 => Err("Modular inverse does not exist".to_string()),
        g => Ok(x % q),
    }
}


pub fn CT_intt(
    ntt_form: &mut Vec<ntt_data_size>,
    n: usize,
    q: ntt_data_size,
    w_inv: ntt_data_size,
    n_inv: ntt_data_size,
    regular_form: &mut Vec<ntt_data_size>,
) -> Result<(), ()> {
    CT_ntt(ntt_form, n, q, w_inv, regular_form).unwrap();
    for x in regular_form.iter_mut() {
        let r: u128 = (*x as u128 * n_inv as u128) % q as u128;
        *x = r as u64;
    }

    Ok(())
}

pub fn CT_ntt(
    regular_form: &mut Vec<ntt_data_size>,
    n: usize,
    q: ntt_data_size,
    w: ntt_data_size,
    ntt_form: &mut Vec<ntt_data_size>,
) -> Result<(), ()> {
    if n == 2 {
        //		xil_printf("CT_ntt 1\n");
        ntt_form[0] = mod_sum(regular_form[0], regular_form[1], q);
        //		xil_printf("CT_ntt 11\n");
        ntt_form[1] = mod_sub(regular_form[0], regular_form[1], q);
        Ok(())
    //		xil_printf("CT_ntt 12\n");

    //		xil_printf("A[0]: %u\n", A[0]);
    //		xil_printf("A[1]: %u\n", A[1]);
    //
    //		xil_printf("B[0]: %u\n", result[0]);
    //		xil_printf("B[1]: %u\n", result[1]);
    } else {
        //		xil_printf("CT_ntt 2\n");
        let N_2 = n / 2;
        let mut B: Vec<ntt_data_size> = Vec::new();
        let mut w_cur = 1;

        for i in 0..n {
            B.push(0);
        }

        let mut A_even: Vec<ntt_data_size> = Vec::new();
        let mut A_odd: Vec<ntt_data_size> = Vec::new();
        let mut B_even: Vec<ntt_data_size> = Vec::new();
        let mut B_odd: Vec<ntt_data_size> = Vec::new();

        for i in 0..N_2 {
            A_even.push(0);
            A_odd.push(0);
            B_even.push(0);
            B_odd.push(0);
        }

        for i in 0..N_2 {
            //		xil_printf("CT_ntt 21\n");
            A_even[i] = regular_form[2 * i];
            A_odd[i] = regular_form[2 * i + 1];
        }

        let w_2: ntt_data_size = mod_mul(w, w, q);

        CT_ntt(&mut A_even, N_2, q, w_2, &mut B_even);
        CT_ntt(&mut A_odd, N_2, q, w_2, &mut B_odd);
        //		xil_printf("CT_ntt 221\n");

        for i in 0..N_2 {
            //			xil_printf("CT_ntt 222: %u\n", i);
            let mut b_i_mul_w: ntt_data_size = mod_mul(w_cur, B_odd[i], q);
            //			xil_printf("CT_ntt 223\n");
            B[i] = mod_sum(B_even[i], b_i_mul_w, q);
            //			xil_printf("CT_ntt 224\n");
            B[i + N_2] = mod_sub(B_even[i], b_i_mul_w, q);
            //			xil_printf("CT_ntt 225\n");
            w_cur = mod_mul(w_cur, w, q);
            //			xil_printf("CT_ntt 226\n");
        }
        //		xil_printf("CT_ntt 227\n");

        for i in 0..n {
            //	xil_printf("CT_ntt 23\n");
            ntt_form[i] = B[i];
        }
        Ok(())
        //		xil_printf("CT_ntt 228\n");
    }
}
