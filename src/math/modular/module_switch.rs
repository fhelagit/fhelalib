#[cfg(test)]
use proptest::prelude::*;

pub fn rounded_div(dividend: u128, divisor: u128) -> u128 {
    // if dividend ^ divisor >= 0 {
    // println!("dividend: {:#x?}", dividend);
    // println!("divisor: {:#x?}", divisor);
    ((dividend).wrapping_add(divisor / 2)).wrapping_div(divisor)
    // } else {
    //     // println!("2");
    //     (dividend.wrapping_sub(divisor / 2)) / divisor
    // }
}

pub fn mod_switch(a: u64, old_q: u128, new_q: u128) -> u64 {
    let nv: u128 = a as u128 * new_q as u128 / old_q as u128;
    // let nv: u128 = a as u128 / rounded_div(old_q as u128, new_q as u128);
    nv as u64
}

#[cfg(test)]
proptest! {
  #![proptest_config(ProptestConfig::with_cases(10000))]
  #[test]
  fn round_div_test(a in 1000..10000u64, b in any::<u64>().prop_filter("Not zero", |v| *v > 100000000000)){
    rounded_div(a as u128, b as u128);
  }
}

#[cfg(test)]
proptest! {
    #![proptest_config(ProptestConfig::with_cases(10000))]
    #[test]
    fn pt_mod_switch_invertible(source in any::<u64>().prop_filter("Not zero", |v| *v > 0)
      , higher_q in 18446744073709551614..18446744073709551615u64
      , lower_q in 18435485074641125377..18446744073709551615u64) {



        let switched = mod_switch(source, higher_q as u128, lower_q as u128);
        // prop_assert!(switched > 0)
        let unswitched = mod_switch(switched, lower_q as u128, higher_q as u128);
       // prop_assert_eq!(unswitched, source);
       let delta = 5;
       prop_assert!(unswitched >= if source.wrapping_sub(delta)>source {0} else {source.wrapping_sub(delta)}  && unswitched < source.wrapping_add(delta), "unswitched: {unswitched}, source: {source} ");

    }
}
