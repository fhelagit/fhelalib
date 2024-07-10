#[allow(non_camel_case_types)]
type ntt_data_size = u64;
#[allow(non_camel_case_types)]
type long_ntt_data_size = u128;

pub fn mod_sum(a: ntt_data_size, b: ntt_data_size, q: ntt_data_size) -> ntt_data_size {
    let c: long_ntt_data_size;
    c = (a as long_ntt_data_size + b as long_ntt_data_size) % q as long_ntt_data_size;
    return c as ntt_data_size;
}

pub fn mod_sub(a: ntt_data_size, b: ntt_data_size, q: ntt_data_size) -> ntt_data_size {
    let c: long_ntt_data_size;
    if a >= b {
        c = (a as long_ntt_data_size - b as long_ntt_data_size) % q as long_ntt_data_size;
    } else {
        c = (q as long_ntt_data_size - (b as long_ntt_data_size - a as long_ntt_data_size))
            % q as long_ntt_data_size;
    }

    return c as ntt_data_size;
}

pub fn mod_mul(a: ntt_data_size, b: ntt_data_size, q: ntt_data_size) -> ntt_data_size {
    let c: long_ntt_data_size;

    c = (a as long_ntt_data_size * b as long_ntt_data_size) % q as long_ntt_data_size;

    return c as ntt_data_size;
}
