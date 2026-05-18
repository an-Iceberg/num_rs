use std::ops::Mul;
use hashbrown::HashMap;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Poly
{
  coefs: HashMap<i64, f64>,
}

impl Eq for Poly
{

}

impl Poly
{
  pub fn from_vec(vec: Vec<(i64, f64)>) -> Self
  {
    let mut poly = Poly::default();
    vec
      .iter()
      // .for_each(λ!{(power, coef) => { poly.coefs.insert(*power, *coef); }});
      .for_each(|(power, coef)| { poly.coefs.insert(*power, *coef); });
    return poly;
  }

  pub fn eval(&self, x: f64) -> f64
  {
    return self.coefs
      .iter()
      .fold(0., |sum, (power, coef)| sum + x.powf(*power as f64) * coef);
  }
}
