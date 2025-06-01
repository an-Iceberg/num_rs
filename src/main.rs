#![allow(non_snake_case)]
#![allow(mixed_script_confusables)]
#![allow(clippy::needless_return)]
#![allow(non_upper_case_globals)]

use std::f64::consts::PI;

use num_rs::{derivative::d, integral::{int_2, int_3, int_4}};

const h: f64 = 0.001;
const a: f64 = 1.;
const b: f64 = 5.;

fn f(x: f64) -> f64 { x.cos() }
fn F(x: f64) -> f64 { x.sin() }

fn main()
{
  println!("Simpon's 1/3: ε = {:.2e}", (int_2(a, b, f, h) - (F(b) - F(a))).abs());
  println!("Simpon's 3/8: ε = {:.2e}", (int_3(a, b, f, h) - (F(b) - F(a))).abs());
  println!("Boole's :     ε = {:.2e}", (int_4(a, b, f, h) - (F(b) - F(a))).abs());
  println!();

  println!("Fläche: {:.4}", int_2(a, b, f, h));

  // fn g(x: f64) -> f64 { (1. + d(f, x, h).powi(2)).sqrt() }
  println!("Bogenlänge: {:.4}", int_2(a, b, |x: f64| (1. + d(f, x, h).powi(2)).sqrt(), h));

  // fn k(x: f64) -> f64 { f(x).powi(2) }
  println!("Rotationsvumen: {:.4}", PI*int_2(a, b, |x: f64| f(x).powi(2), h));
}
