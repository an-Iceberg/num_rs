#![allow(non_snake_case)]
#![allow(mixed_script_confusables)]
#![allow(clippy::needless_return)]
#![allow(non_upper_case_globals)]

use std::{f64::consts::PI, ops::Neg};

use num_rs::{derivative::{d, d2}, integral::{int_2, int_3, int_4, int_6}, λ};

const h: f64 = 0.01;
const a: f64 = 1.24;
const b: f64 = 5.38;
const π: f64 = PI;

fn F(x: f64) -> f64 { x.sin() }
fn f(x: f64) -> f64 { x.cos() }
fn df(x: f64) -> f64 { x.sin().neg() }
fn d2f(x: f64) -> f64 { x.cos().neg() }


fn main()
{
  println!("Simpon's 1/3: ε = {:.2e}", (int_2(a, b, f, h) - (F(b) - F(a))).abs());
  println!("Simpon's 3/8: ε = {:.2e}", (int_3(a, b, f, h) - (F(b) - F(a))).abs());
  println!("Boole's :     ε = {:.2e}", (int_4(a, b, f, h) - (F(b) - F(a))).abs());
  println!("Weddle's :    ε = {:.2e}", (int_6(a, b, f, h) - (F(b) - F(a))).abs());
  println!();

  println!("Fläche: {:.4}", int_2(a, b, f, h));
  println!("Bogenlänge: {:.4}", int_2(a, b, λ!{x => (1. + d(f, x, h).powi(2)).sqrt()}, h));
  println!("Rotationsvumen: {:.4}", π*int_2(a, b, λ!{x => f(x).powi(2)}, h));
  println!();

  println!("d(f): ε = {:.2e}", (df(5.) - d(f, 5., 0.001)).abs());
  println!("d2(f): ε = {:.2e}", (d2f(5.) - d2(f, 5., 0.001)).abs());
}
