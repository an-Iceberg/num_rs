#![allow(non_snake_case)]
#![allow(mixed_script_confusables)]

use num_rs::integral::{int_2, int_3, int_4};

fn main()
{
  let f = |x: f64| x.cos();
  let F = |x: f64| x.sin();
  let a = 1.;
  let b = 5.;
  let h = 0.001;

  println!("Simpon's 1/3: ε = {:.2e}", (int_2(a, b, f, h) - (F(b) - F(a))).abs());
  println!("Simpon's 3/8: ε = {:.2e}", (int_3(a, b, f, h) - (F(b) - F(a))).abs());
  println!("Boole's :     ε = {:.2e}", (int_4(a, b, f, h) - (F(b) - F(a))).abs());
}
