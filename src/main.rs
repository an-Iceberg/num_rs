#![allow(non_snake_case)]
#![allow(mixed_script_confusables)]
#![allow(clippy::needless_return)]
#![allow(non_upper_case_globals)]

use std::{f64::consts::PI, ops::Neg};
use num_rs::{derivative::{d, d2}, integral::{int_2, int_3, int_4, int_6}, λ};

fn main()
{
  let h = 0.01;
  let a = 1.24;
  let b = 5.38;
  let π = PI;

  let F = λ! {x: f64 => x.sin()};
  let f = λ! {x: f64 => x.cos()};
  let df = λ! {x: f64 => x.sin().neg()};
  let d2f = λ! {x: f64 => x.cos().neg()};

  println!("Simpon's 1/3: ε = {:.2e}", (int_2(a, b, f, h) - (F(b) - F(a))).abs());
  println!("Simpon's 3/8: ε = {:.2e}", (int_3(a, b, f, h) - (F(b) - F(a))).abs());
  println!("Boole's :     ε = {:.2e}", (int_4(a, b, f, h) - (F(b) - F(a))).abs());
  println!("Weddle's :    ε = {:.2e}", (int_6(a, b, f, h) - (F(b) - F(a))).abs());
  println!();

  // Area
  println!("Fläche: {:.4}", int_2(a, b, f, h));
  // Arc length
  println!("Bogenlänge: {:.4}", int_2(a, b, λ!{x => (1. + d(f, x, h).powi(2)).sqrt()}, h));
  // Rotation(al) volume
  println!("Rotationsvumen: {:.4}", π*int_2(a, b, λ!{x => f(x).powi(2)}, h));
  println!();

  println!("d(f): ε = {:.2e}", (df(5.) - d(f, 5., 0.001)).abs());
  println!("d2(f): ε = {:.2e}", (d2f(5.) - d2(f, 5., 0.001)).abs());

  // A nice way to do this.
  let f = λ!{x: f64 => x.powi(2) + 2.*x + x.cos()};
  let precision = 0.001;
  dbg!{format!("{:.5}", d(f, 5., precision))};
  dbg!{format!("{:.5}", int_2(0., 10., f, precision))};
  dbg!{format!("{:.5}", int_2(-5., -2., f, precision))};
  dbg!{format!("{:.5}", int_3(-5., -2., f, precision))};
  dbg!{format!("{:.5}", int_4(-5., -2., f, precision))};
  dbg!{format!("{:.5}", int_6(-5., -2., f, precision))};
}
