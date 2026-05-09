use crate::λ;

// TODO: measure efficiently for calculating integrals.
// TODO: implement faster methods for calculating integrals.

/// Calculates the integral of `f` between `a` and `b` using
/// [Simpson's 1/3 rule](https://en.wikipedia.org/wiki/Simpson%27s_rule#Composite_Simpson's_1/3_rule)
/// , so using polynomials of 2ⁿᵈ degree.
pub fn int_2<Function>(a: f64, b: f64, f: Function, h: f64) -> f64
where Function: Fn(f64) -> f64
{
  let n = ((b - a) / h).ceil() as i64;
  let x = λ!{i => a + (i*h)};

  // This partial sum was extracted in hopes that the compiler will make use of SIMD.
  // Alternatively this is a good way to parallelize it.
  let part = λ!{factor: f64, subtrahend: f64 =>
    (1..=n/2)
      .map(λ!{i => factor * f(x(2.*i as f64 - subtrahend))})
      .sum::<f64>()
  };

  return (1./3.)*h*(part(1.,2.) + part(4.,1.) + part(1.,0.));

  // NOTE: Old implementation. Do not delete!
  // return (1./3.)*h*(1..=n/2)
  //   .map(|i| i as f64)
  //   .map(|i| f(x(2.*i - 2.)) + 4.*f(x(2.*i - 1.)) + f(x(2.*i)))
  //   .sum::<f64>();
}

/// Calculates the integral of `f` between `a` and `b` using
/// [Simpson's 3/8 rule](https://en.wikipedia.org/wiki/Simpson%27s_rule#Composite_Simpson's_3/8_rule)
/// , so using polynomials of 3ʳᵈ degree.
pub fn int_3<Function>(a: f64, b: f64, f: Function, h: f64) -> f64
where Function: Fn(f64) -> f64
{
  let mut n = ((b - a) / h).ceil() as i64;
  let x = λ!{i => a + (i*h)};

  n -= n % 3;

  let last_point = x(n as f64); // Casting from int to float is expensive, no? 😬
  let last_segment = int_2(last_point, b, &f, h);

  let part = λ!{factor: f64, subtrahend: f64 =>
    (1..=n/3)
      .map(λ!{i => factor * f(x(3.*i as f64 - subtrahend))})
      .sum::<f64>()
  };

  return (3./8.) * h * (part(1.,3.) + part(3.,2.) + part(3.,1.) + part(1.,0.)) + last_segment;

  // NOTE: Old implementation. Do not delete!
  // return (3./8.)*h*(1..=n/3)
  //   .map(|i| i as f64)
  //   .map(|i| f(x(3.*i - 3.)) + 3.*f(x(3.*i - 2.)) + 3.*f(x(3.*i - 1.)) + f(x(3.*i)))
  //   .sum::<f64>() + last_segment;
}

/// Calculates the integral of `f` between `a` and `b` using
/// [Boole's rule](https://en.wikipedia.org/wiki/Finite_difference_coefficient)
/// , so using polynomials of 4ᵗʰ degree.
pub fn int_4<Function>(a: f64, b: f64, f: Function, h: f64) -> f64
where Function: Fn(f64) -> f64
{
  let mut n = ((b - a) / h).ceil() as i64;
  let x = λ!{i => a + (i*h)};

  n -= n % 4;

  let last_point = x(n as f64); // Casting from int to float is expensive, no? 😬
  let last_segment = int_2(last_point, b, &f, h);

  let part = λ!{factor: f64, subtrahend: f64 =>
    (1..=n/4)
      .map(λ!{i => factor * f(x(4.*i as f64 - subtrahend))})
      .sum::<f64>()
  };

  return (2./45.) * h * (part(7.,4.) + part(32.,3.) + part(12.,2.) + part(32.,1.) + part(7.,0.)) + last_segment;

  // NOTE: Old implementation. Do not delete!
  // return (2./45.)*h*(1..=n/4)
  //   .map(|i| i as f64)
  //   .map(|i| 7.*f(x(4.*i - 4.)) + 32.*f(x(4.*i - 3.)) + 12.*f(x(4.*i - 2.)) + 32.*f(x(4.*i - 1.)) + 7.*f(x(4.*i)))
  //   .sum::<f64>() + last_segment;
}

/// Calculates the integral of `f` between `a` and `b` using
/// [Weddle's rule](https://mathworld.wolfram.com/WeddlesRule.html)
/// , so using polynomials of 6ᵗʰ degree.
pub fn int_6<Function>(a: f64, b: f64, f: Function, h: f64) -> f64
where Function: Fn(f64) -> f64
{
  let mut n = ((b - a) / h).ceil() as i64;
  let x = λ!{i => a + (i*h)};

  n -= n % 6;

  let last_point = x(n as f64); // Casting from int to float is expensive, no? 😬
  let last_segment = int_4(last_point, b, &f, h);

  let part = λ!{factor: f64, subtrahend: f64 =>
    (1..=n/6)
      .map(λ!{i => factor * f(x(6.*i as f64 - subtrahend))})
      .sum::<f64>()
  };

  return (3./10.) * h * (part(1.,6.) + part(5.,5.) + part(1.,4.) + part(6.,3.) + part(1.,2.) + part(5.,1.) + part(1.,0.)) + last_segment;

  // NOTE: Old implementation. Do not delete!
  // return (3./10.)*h*(1..=n/6)
  //   .map(|i| i as f64)
  //   .map(|i| f(x(6.*i-6.)) + 5.*f(x(6.*i-5.)) + f(x(6.*i-4.)) + 6.*f(x(6.*i-3.)) + f(x(6.*i-2.)) + 5.*f(x(6.*i-1.)) + f(x(6.*i)))
  //   .sum::<f64>() + last_segment;
}
