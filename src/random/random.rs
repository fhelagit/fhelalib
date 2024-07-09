use rand::Rng;
use rand_distr::{Normal, Distribution};

pub fn rnd_u64_uniform() -> u64 {
  let mut rng = rand::thread_rng();
  rng.gen_range(0..=u64::MAX)
}

pub fn rnd_u64_uniform_bounded(bound: u64) -> u64 {
  let mut rng = rand::thread_rng();
  rng.gen_range(0..bound)
}

pub fn rnd_u64_uniform_binary() -> u64 {
  rnd_u64_uniform_bounded(2)
}


pub fn rnd_u64_gausean() -> u64 {
  let dist = Normal::new(0.0, 3.0).unwrap();
  let mut rng = rand::thread_rng();
  let sampled = dist.sample(&mut rng);
  if round(sampled, 0) >= 0.0 {
    return round(sampled, 0) as u64
  } else {
    return u64::MAX - round(-sampled, 0) as u64 + 1
  } 
}
