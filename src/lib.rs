#![allow(clippy::needless_return)]
#![allow(mixed_script_confusables)]
#![allow(non_snake_case)]

/// Calculates the integral of `f` between `a` and `b` using
/// [Simpson's ⅓ rule](https://en.wikipedia.org/wiki/Simpson%27s_rule#Composite_Simpson's_1/3_rule)
/// , so using polynomials of 2ⁿᵈ degree.
pub fn int_2(a: f64, b: f64, f: fn(f64) -> f64, h: f64) -> f64
{
  let n = ((b - a) / h).ceil() as i64;
  let x = |i: f64| a +(i * h);

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
  let n = ((b - a) / h).ceil() as i64;
  let x = |i: f64| a +(i * h);

  return (3./8.)*h*(1..=n/3)
    .map(|i| i as f64)
    .map(|i| f(x(3.*i - 3.)) + 3.*f(x(3.*i - 2.)) + 3.*f(x(3.*i - 1.)) + f(x(3.*i)))
    .sum::<f64>();
}

/// Calculates the integral of `f` between `a` and `b` using
/// [Boole's rule](https://en.wikipedia.org/wiki/Finite_difference_coefficient)
/// , so using polynomials of 4ᵗʰ degree.
pub fn int_4(a: f64, b: f64, f: fn(f64) -> f64, h: f64) -> f64
{
  let n = ((b - a) / h).ceil() as i64;
  let x = |i: f64| a +(i * h);

  return (2./45.)*h*(1..=n/4)
    .map(|i| i as f64)
    .map(|i| 7.*f(x(4.*i - 4.)) + 32.*f(x(4.*i - 3.)) + 12.*f(x(4.*i - 2.)) + 32.*f(x(4.*i - 1.)) + 7.*f(x(4.*i)))
    .sum::<f64>();
}

#[cfg(test)]
mod tests
{
  use super::*;

  const ERROR: f64 = 1e-5;

  mod int_2 {
    use crate::tests::{int_2, ERROR};

    const A: f64 = 1.;
    const B: f64 = 5.;
    const H: f64 = 0.001;

    #[test]
    fn test_1() {
      let f = |x: f64| x.cos();
      let F = |x: f64| x.sin();

      let analytical_solution = F(B) - F(A);
      let numerical_solution = int_2(A, B, f, H);
      let ε = (numerical_solution - analytical_solution).abs();

      assert!(ε < ERROR, "ε = {:.2e}, should = {:.2e}", ε, ERROR);
    }

    #[test]
    fn test_2() {
      let f = |x: f64| 1./x;
      let F = |x: f64| x.abs().ln();

      let analytical_solution = F(B) - F(A);
      let numerical_solution = int_2(A, B, f, H);
      let ε = (numerical_solution - analytical_solution).abs();

      assert!(ε < ERROR, "ε = {:.2e}, should = {:.2e}", ε, ERROR);
    }

    #[test]
    fn test_3() {
      let f = |x: f64| x.sqrt();
      let F = |x: f64| (2./3.)*x*x.sqrt();

      let analytical_solution = F(B) - F(A);
      let numerical_solution = int_2(A, B, f, H);
      let ε = (numerical_solution - analytical_solution).abs();

      assert!(ε < ERROR, "ε = {:.2e}, should = {:.2e}", ε, ERROR);
    }

    #[test]
    fn test_4() {
      let f = |x: f64| x.exp();
      let F = |x: f64| x.exp();

      let analytical_solution = F(B) - F(A);
      let numerical_solution = int_2(A, B, f, H);
      let ε = (numerical_solution - analytical_solution).abs();

      assert!(ε < ERROR, "ε = {:.2e}, should = {:.2e}", ε, ERROR);
    }
  }

  mod int_3 {
    use crate::tests::{int_3, ERROR};

    const A: f64 = 1.;
    const B: f64 = 5.;
    const H: f64 = 0.001;

    #[test]
    fn test_1() {
      let f = |x: f64| x.cos();
      let F = |x: f64| x.sin();

      let analytical_solution = F(B) - F(A);
      let numerical_solution = int_3(A, B, f, H);
      let ε = (numerical_solution - analytical_solution).abs();

      assert!(ε < ERROR, "ε = {:.2e}, should = {:.2e}", ε, ERROR);
    }

    #[test]
    fn test_2() {
      let f = |x: f64| 1./x;
      let F = |x: f64| x.abs().ln();

      let analytical_solution = F(B) - F(A);
      let numerical_solution = int_3(A, B, f, H);
      let ε = (numerical_solution - analytical_solution).abs();

      assert!(ε < ERROR, "ε = {:.2e}, should = {:.2e}", ε, ERROR);
    }

    #[test]
    fn test_3() {
      let f = |x: f64| x.sqrt();
      let F = |x: f64| (2./3.)*x*x.sqrt();

      let analytical_solution = F(B) - F(A);
      let numerical_solution = int_3(A, B, f, H);
      let ε = (numerical_solution - analytical_solution).abs();

      assert!(ε < ERROR, "ε = {:.2e}, should = {:.2e}", ε, ERROR);
    }

    #[test]
    fn test_4() {
      let f = |x: f64| x.exp();
      let F = |x: f64| x.exp();

      let analytical_solution = F(B) - F(A);
      let numerical_solution = int_3(A, B, f, H);
      let ε = (numerical_solution - analytical_solution).abs();

      assert!(ε < ERROR, "ε = {:.2e}, should = {:.2e}", ε, ERROR);
    }
  }

  mod int_4 {
    use crate::tests::{int_4, ERROR};

    const A: f64 = 1.;
    const B: f64 = 5.;
    const H: f64 = 0.001;

    #[test]
    fn test_1() {
      let f = |x: f64| x.cos();
      let F = |x: f64| x.sin();

      let analytical_solution = F(B) - F(A);
      let numerical_solution = int_4(A, B, f, H);
      let ε = (numerical_solution - analytical_solution).abs();

      assert!(ε < ERROR, "ε = {:.2e}, should = {:.2e}", ε, ERROR);
    }

    #[test]
    fn test_2() {
      let f = |x: f64| 1./x;
      let F = |x: f64| x.abs().ln();

      let analytical_solution = F(B) - F(A);
      let numerical_solution = int_4(A, B, f, H);
      let ε = (numerical_solution - analytical_solution).abs();

      assert!(ε < ERROR, "ε = {:.2e}, should = {:.2e}", ε, ERROR);
    }

    #[test]
    fn test_3() {
      let f = |x: f64| x.sqrt();
      let F = |x: f64| (2./3.)*x*x.sqrt();

      let analytical_solution = F(B) - F(A);
      let numerical_solution = int_4(A, B, f, H);
      let ε = (numerical_solution - analytical_solution).abs();

      assert!(ε < ERROR, "ε = {:.2e}, should = {:.2e}", ε, ERROR);
    }

    #[test]
    fn test_4() {
      let f = |x: f64| x.exp();
      let F = |x: f64| x.exp();

      let analytical_solution = F(B) - F(A);
      let numerical_solution = int_4(A, B, f, H);
      let ε = (numerical_solution - analytical_solution).abs();

      assert!(ε < ERROR, "ε = {:.2e}, should = {:.2e}", ε, ERROR);
    }
  }
}
