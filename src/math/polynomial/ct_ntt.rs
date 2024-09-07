#[allow(non_camel_case_types)]
type ntt_data_size = u64;
use cached::proc_macro::cached;
use crate::math::modular::mod_arith::*;
#[cfg(test)]
use proptest::prelude::*;
#[cfg(test)]
// use proptest_derive::Arbitrary;
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

        ct_ntt(&mut regular_form, n,  q, w, &mut nnt_form).unwrap();
        println!("1. reg_form: {:?}, \n   ntt_form: {:?}", regular_form, nnt_form);
        ct_intt(&mut nnt_form, n,  q, w_inv, n_inv, &mut regular_form).unwrap();
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

    ct_ntt(&mut regular_form, n, q, w, &mut ntt_form).unwrap();
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
    let expected_regular_form = [1; n].to_vec();

    ct_intt(&mut ntt_form, n, q, w_inv, n_inv, &mut regular_form).unwrap();
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

        ct_ntt(&mut regular_form, n,  q, w, &mut nnt_form).unwrap();
        println!("1. reg_form: {:?}, \n   ntt_form: {:?}", regular_form, nnt_form);
        ct_intt(&mut nnt_form, n,  q, w_inv, n_inv, &mut regular_form).unwrap();
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

    ct_ntt(&mut regular_form, n, q, w, &mut ntt_form).unwrap();
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
    let expected_regular_form = [1; n].to_vec();

    ct_intt(&mut ntt_form, n, q, w_inv, n_inv, &mut regular_form).unwrap();
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

#[cached]
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
    let (g, x, _) = egcd(a, q);
    match g {
        1 => Err("Modular inverse does not exist".to_string()),
        _ => Ok(x % q),
    }
}

#[inline(always)]
pub fn ct_intt(
    ntt_form: &mut Vec<ntt_data_size>,
    n: usize,
    q: ntt_data_size,
    w_inv: ntt_data_size,
    n_inv: ntt_data_size,
    regular_form: &mut Vec<ntt_data_size>,
) -> Result<(), ()> {
    ct_ntt(ntt_form, n, q, w_inv, regular_form).unwrap();
    for x in regular_form.iter_mut() {
        let r: u128 = (*x as u128 * n_inv as u128) % q as u128;
        *x = r as u64;
    }

    Ok(())
}

#[inline(always)]
pub fn ct_ntt(
    regular_form: &mut Vec<ntt_data_size>,
    n: usize,
    q: ntt_data_size,
    w: ntt_data_size,
    ntt_form: &mut Vec<ntt_data_size>,
) -> Result<(), ()> {
    iter_dit_ntt(regular_form, n, q, w, ntt_form)
    // if n == 2 {
    //     //		xil_printf("CT_ntt 1\n");
    //     ntt_form.push(mod_sum(regular_form[0], regular_form[1], q));
    //     //		xil_printf("CT_ntt 11\n");
    //     ntt_form.push(mod_sub(regular_form[0], regular_form[1], q));
    //     Ok(())
    // //		xil_printf("CT_ntt 12\n");

    // //		xil_printf("A[0]: %u\n", A[0]);
    // //		xil_printf("A[1]: %u\n", A[1]);
    // //
    // //		xil_printf("B[0]: %u\n", result[0]);
    // //		xil_printf("B[1]: %u\n", result[1]);
    // } else {
    //     //		xil_printf("CT_ntt 2\n");
    //     let n_2 = n / 2;
    //     // let mut b: Vec<ntt_data_size> = Vec::new();
    //     let mut w_cur = 1;

    //     // for _ in 0..n {
    //     //     ntt_form.push(0);
    //     // }

    //     let mut a_even: Vec<ntt_data_size> = Vec::with_capacity(n_2);
    //     let mut a_odd: Vec<ntt_data_size> = Vec::with_capacity(n_2);
    //     let mut b_even: Vec<ntt_data_size> = Vec::with_capacity(n_2);
    //     let mut b_odd: Vec<ntt_data_size> = Vec::with_capacity(n_2);

    //     // for _ in 0..n_2 {
    //     //     // a_even.push(0);
    //     //     // a_odd.push(0);
    //     //     b_even.push(0);
    //     //     b_odd.push(0);
    //     // }

    //     for i in 0..n_2 {
    //         //		xil_printf("CT_ntt 21\n");
    //         ntt_form.push(0);
    //         ntt_form.push(0);
    //         a_even.push(regular_form[2 * i]);
    //         a_odd.push(regular_form[2 * i + 1]);
    //     }

    //     let w_2: ntt_data_size = mod_mul(w, w, q);

    //     ct_ntt(&mut a_even, n_2, q, w_2, &mut b_even)?;
    //     ct_ntt(&mut a_odd, n_2, q, w_2, &mut b_odd)?;
    //     //		xil_printf("CT_ntt 221\n");

    //     for i in 0..n_2 {
    //         //			xil_printf("CT_ntt 222: %u\n", i);
    //         let b_i_mul_w: ntt_data_size = mod_mul(w_cur, b_odd[i], q);
    //         //			xil_printf("CT_ntt 223\n");
    //         ntt_form[i] = mod_sum(b_even[i], b_i_mul_w, q);
    //         //			xil_printf("CT_ntt 224\n");
    //         ntt_form[i + n_2] = mod_sub(b_even[i], b_i_mul_w, q);
    //         //			xil_printf("CT_ntt 225\n");
    //         w_cur = mod_mul(w_cur, w, q);
    //         //			xil_printf("CT_ntt 226\n");
    //     }
    //     //		xil_printf("CT_ntt 227\n");

    //     // for i in 0..n {
    //     //     //	xil_printf("CT_ntt 23\n");
    //     //     ntt_form.push(b[i]);
    //     // }
    //     Ok(())
    //     //		xil_printf("CT_ntt 228\n");
    // }
}

pub fn iter_dit_ntt(
        regular_form: &mut Vec<ntt_data_size>,
        n: usize,
        q: ntt_data_size,
        w: ntt_data_size,
        ntt_form: &mut Vec<ntt_data_size>,
    ) -> Result<(), ()> {

    //		xil_printf("CT_ntt 2\n");
    // let mut C: Vec<u64> = Vec::with_capacity(n);
    let mut B: Vec<ntt_data_size> = Vec::with_capacity(n);
    let n_2 = n>>1;
    for i in 0..n {
        // ntt_form.push(0);
        ntt_form.push(regular_form[i]);
        B.push(regular_form[i]);
    }

    let mut v = n>>1;
    let mut m = 1;
    let mut d = n>>1;

    let mut nsi = if ((v as f64).log2() as u64) % 2 == 0 {true} else {false};

    while m < n {
        if nsi {
            let mut l = 0;
            for k in 0..m {
                let jf = 2*k*v;
                let jl = jf + v - 1;
                let jt = k*v;

                let tw = pow(w, jt as u32, q);

                for j in jf..jl+1 {
                    let temp        = mod_mul(tw, B[j+d], q);

                    ntt_form[l]     = mod_sum(B[j], temp, q);
                    ntt_form[l+n_2] = mod_sub(B[j], temp, q);

                    l += 1
                }
            }
            nsi = false
                

        } else {
            let mut l = 0;
            for k in 0..m{
                let jf = 2*k*v;
                let jl = jf + v - 1;
                let jt = k*v;
    
                let tw = pow(w, jt as u32, q);
    
                for j in jf..jl+1 {
                    let temp = mod_mul(tw, ntt_form[j+d], q);
    
                    B[l]          = mod_sum(ntt_form[j], temp, q);
                    B[l+n_2] = mod_sub(ntt_form[j], temp, q);
    
                    l += 1
                }
            }
            nsi = true
            
        }
        v >>= 1;
        m <<= 1;
        d >>= 1;
    }
    Ok(())

}
