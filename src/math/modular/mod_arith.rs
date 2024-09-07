

#[allow(non_camel_case_types)]
type ntt_data_size = u64;
#[allow(non_camel_case_types)]
type long_ntt_data_size = u128;

#[inline(always)]
pub fn mod_sum(a: ntt_data_size, b: ntt_data_size, q: ntt_data_size) -> ntt_data_size {

    let neg_b = q - b;
    if a >= neg_b {
        a - neg_b
    } else {
        a + b
    }

}

#[inline(always)]
pub fn mod_sub(a: ntt_data_size, b: ntt_data_size, q: ntt_data_size) -> ntt_data_size {

    let neg_b = q - b;
    if a >= b {
        a - b
    } else {
        a + neg_b
    }

}

#[inline(always)]
pub fn mod_mul(a: ntt_data_size, b: ntt_data_size, m: ntt_data_size) -> ntt_data_size {

    let c: long_ntt_data_size;

    c = (a as long_ntt_data_size * b as long_ntt_data_size) % m as long_ntt_data_size;

    return c as ntt_data_size;
}
