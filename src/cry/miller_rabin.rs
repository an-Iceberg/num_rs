use crate::cry::pow_mod::pow_mod;
use rand::random_range;

fn miller_rabin_test(n: u64, d: u64, a: u64) -> bool
{
  let mut d = d;
  let mut x = pow_mod(a, d, n);
  if x == 1 || x == n - 1 { return true; }

  while d != n - 1
  {
    x = pow_mod(x, 2, n);
    d *= 2;
    if x == 1 { return false; }
    if x == n - 1 { return true; }
  }

  return false;
}

pub fn is_prime(n: u64, passes: u64) -> bool
{
  if n == 2 || n == 3 || n == 5 || n == 7 { return true; }
  if n == 1 || n == 4 || n == 6 { return false; }
  if n.is_multiple_of(2) || n.is_multiple_of(3) || n.is_multiple_of(4) { return false; }

  let mut d = n - 1;
  while d.is_multiple_of(2) { d /= 2; }

  for _ in 1..=passes
  {
    let a = random_range(2..n-4);
    if !miller_rabin_test(n, d, a) { return false; }
  }

  return true;
}

#[cfg(test)]
mod tests
{
  use super::super::miller_rabin;

  const PASSES: u64 = 20;
  #[test] fn test_1() { assert!(miller_rabin::is_prime(6967, PASSES)); }
  #[test] fn test_2() { assert!(miller_rabin::is_prime(7919, PASSES)); }
  #[test] fn test_3() { assert!(miller_rabin::is_prime(3889, PASSES)); }
  #[test] fn test_4() { assert!(!miller_rabin::is_prime(6966, PASSES)); }
  #[test] fn test_5() { assert!(!miller_rabin::is_prime(7920, PASSES)); }
  #[test] fn test_6() { assert!(!miller_rabin::is_prime(3888, PASSES)); }
  #[test] fn test_7() { assert!(!miller_rabin::is_prime(3887, PASSES)); }
}
