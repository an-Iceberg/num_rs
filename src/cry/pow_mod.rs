use crate::λ;

// Source: https://stackoverflow.com/questions/30226094/how-do-i-decompose-a-number-into-powers-of-2#answer-30227161
fn powers_of_2_(n: u64) -> Vec<u64>
{
  let mut powers = vec![];
  let mut i = 1;
  while i <= n
  {
    if i & n == 1 { powers.push(i); }
    i <<= 1;
  }
  return powers;
}

// Source: https://stackoverflow.com/questions/8898807/pythonic-way-to-iterate-over-bits-of-integer#answer-8898977
fn powers_of_2(n: u64) -> Vec<u64>
{
  let mut n = n;
  let mut bits = vec![];
  while n != 0b0
  {
    let b = n & (!n + 1);
    bits.push(b);
    n ^= b;
  }
  return bits;
}

fn log_2(factor: u64) -> u64
{
  let mut factor = factor;
  let mut counter = 0;
  while factor != 0b1
  {
    factor >>= 1;
    counter += 1;
  }
  return counter;
}

pub fn pow_mod(base: u64, exponent: u64, modulus: u64) -> u64
{
  let mut result = 1;
  let mut block_result = base;
  let exponents = powers_of_2(exponent)
    .iter()
    .map(λ!{exp => log_2(*exp)})
    .collect::<Vec<u64>>();
  let mut previous_k = 0;
  for k in exponents
  {
    for _ in 1..=k - previous_k
    {
      block_result = block_result.pow(2);
      block_result %= modulus;
    }
    result *= block_result;
    result %= modulus;
    previous_k = k;
  }

  return result;
}

#[cfg(test)]
mod tests
{
  use super::pow_mod;

  #[test] fn test_1() { assert_eq!(0, pow_mod(100, 10, 50)); }
  #[test] fn test_2() { assert_eq!(4, pow_mod(100, 10, 51)); }
  #[test] fn test_3() { assert_eq!(48, pow_mod(100, 10, 52)); }
  #[test] fn test_4() { assert_eq!(13, pow_mod(100, 10, 53)); }
  #[test] fn test_5() { assert_eq!(46, pow_mod(100, 10, 54)); }
  #[test] fn test_6() { assert_eq!(7, pow_mod(123, 7, 52)); }
  #[test] fn test_7() { assert_eq!(17, pow_mod(98, 23, 45)); }
  #[test] fn test_8() { assert_eq!(16, pow_mod(1032, 17, 142)); }
  #[test] fn test_9() { assert_eq!(7, pow_mod(7, 25, 73)); }
}
