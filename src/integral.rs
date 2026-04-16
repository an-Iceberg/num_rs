use crate::λ;

// TODO: measure efficiently for calculating integrals.
// TODO: implement faster methods for calculating integrals.

/// Calculates the integral of `f` between `a` and `b` using
/// [Simpson's ⅓ rule](https://en.wikipedia.org/wiki/Simpson%27s_rule#Composite_Simpson's_1/3_rule)
/// , so using polynomials of 2ⁿᵈ degree.
pub fn int_2(a: f64, b: f64, f: fn(f64) -> f64, h: f64) -> f64
{
  let n = ((b - a) / h).ceil() as i64;
  let x = λ!{i => a + (i*h)};

  return (1./3.)*h*(1..=n/2)
    .map(|i| i as f64)
    .map(|i| f(x(2.*i - 2.)) + 4.*f(x(2.*i - 1.)) + f(x(2.*i)))
    .sum::<f64>();
}

/// Calculates the integral of `f` between `a` and `b` using
/// [Simpson's ⅜ rule](https://en.wikipedia.org/wiki/Simpson%27s_rule#Composite_Simpson's_3/8_rule)
/// , so using polynomials of 3ʳᵈ degree.
pub fn int_3(a: f64, b: f64, f: fn(f64) -> f64, h: f64) -> f64
{
  let mut n = ((b - a) / h).ceil() as i64;
  let x = |i: f64| a +(i * h);

  n -= n % 3;

  let last_point = x(n as f64); // Casting from int to float is expensive, no? 😬
  let last_segment = int_2(last_point, b, f, h);

  return (3./8.)*h*(1..=n/3)
    .map(|i| i as f64)
    .map(|i| f(x(3.*i - 3.)) + 3.*f(x(3.*i - 2.)) + 3.*f(x(3.*i - 1.)) + f(x(3.*i)))
    .sum::<f64>() + last_segment;
}

/// Calculates the integral of `f` between `a` and `b` using
/// [Boole's rule](https://en.wikipedia.org/wiki/Finite_difference_coefficient)
/// , so using polynomials of 4ᵗʰ degree.
pub fn int_4(a: f64, b: f64, f: fn(f64) -> f64, h: f64) -> f64
{
  let mut n = ((b - a) / h).ceil() as i64;
  let x = |i: f64| a +(i * h);

  n -= n % 4;

  let last_point = x(n as f64); // Casting from int to float is expensive, no? 😬
  let last_segment = int_2(last_point, b, f, h);

  return (2./45.)*h*(1..=n/4)
    .map(|i| i as f64)
    .map(|i| 7.*f(x(4.*i - 4.)) + 32.*f(x(4.*i - 3.)) + 12.*f(x(4.*i - 2.)) + 32.*f(x(4.*i - 1.)) + 7.*f(x(4.*i)))
    .sum::<f64>() + last_segment;
}

/// Calculates the integral of `f` between `a` and `b` using
/// [Weddle's rule](https://mathworld.wolfram.com/WeddlesRule.html)
/// , so using polynomials of 6ᵗʰ degree.
pub fn int_6(a: f64, b: f64, f: fn(f64) -> f64, h: f64) -> f64
{
  let mut n = ((b - a) / h).ceil() as i64;
  let x = |i: f64| a +(i * h);

  n -= n % 6;

  let last_point = x(n as f64); // Casting from int to float is expensive, no? 😬
  let last_segment = int_4(last_point, b, f, h);

  return (3./10.)*h*(1..=n/6)
    .map(|i| i as f64)
    .map(|i| f(x(6.*i-6.)) + 5.*f(x(6.*i-5.)) + f(x(6.*i-4.)) + 6.*f(x(6.*i-3.)) + f(x(6.*i-2.)) + 5.*f(x(6.*i-1.)) + f(x(6.*i)))
    .sum::<f64>() + last_segment;
}
