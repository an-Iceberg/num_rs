#![allow(clippy::needless_return)]
#![allow(mixed_script_confusables)]
#![allow(non_snake_case)]
#![allow(dead_code)]

const ERROR: f64 = 1e-5;

#[cfg(test)]
mod int_2 {
  use num_rs::integral::int_2;
  use crate::ERROR;

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

#[cfg(test)]
mod int_3 {
  use num_rs::integral::int_3;
  use crate::ERROR;

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

#[cfg(test)]
mod int_4 {
  use num_rs::integral::int_4;
  use crate::ERROR;

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
