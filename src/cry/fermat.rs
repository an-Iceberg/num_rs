use rand::random_range;
use crate::cry::pow_mod::pow_mod;

fn fermat_test(n: u64, a: u64) -> bool
{
  return pow_mod(a, n-1, n) == 1;
}

pub fn is_prime(n: u64, passes: u64) -> bool
{
  if n == 2 || n == 3 || n == 5 || n == 7 { return true; }
  if n == 1 || n == 4 || n == 6 { return false; }
  if n.is_multiple_of(2) || n.is_multiple_of(3) || n.is_multiple_of(4) { return false; }

  for _ in 1..=passes
  {
    let a = random_range(2..n-2);
    if !fermat_test(n, a) { return false; }
  }

  return true;
}

#[cfg(test)]
mod tests
{
  use super::super::fermat;

  const PASSES: u64 = 20;
  #[test] fn test_1() { assert!(fermat::is_prime(6967, PASSES)); }
  #[test] fn test_2() { assert!(fermat::is_prime(7919, PASSES)); }
  #[test] fn test_3() { assert!(fermat::is_prime(3889, PASSES)); }
  #[test] fn test_4() { assert!(!fermat::is_prime(6966, PASSES)); }
  #[test] fn test_5() { assert!(!fermat::is_prime(7920, PASSES)); }
  #[test] fn test_6() { assert!(!fermat::is_prime(3888, PASSES)); }
  #[test] fn test_7() { assert!(!fermat::is_prime(3887, PASSES)); }
}
