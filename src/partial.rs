// ∂¹

pub fn part1_2(f: fn(Vec<f64>) -> f64, x: Vec<f64>, i: usize, h: f64) -> f64
{
  // Note: this is inefficient b/c it requires a ton of extra memory (but it might still be fast)

  let mut a = x.clone();
  let mut b = x.clone();

  a[i] -= h;
  b[i] += h;

  return (-0.5*f(a) + 0.5*f(b)) / h;
}

pub fn part1_4(f: fn(&Vec<f64>) -> f64, x: Vec<f64>, i: usize, h: f64) -> f64
{
  let mut a = x.clone();
  let mut b = x.clone();
  let mut c = x.clone();
  let mut d = x.clone();

  a[i] -= 2.*h;
  b[i] -= h;
  c[i] += h;
  d[i] += 2.*h;

  return ((1./12.)*f(&a) - (2./3.)*f(&b) + (2./3.)*f(&c) - (1./12.)*f(&d)) / h;
}

// ∂²

// ∂³

// ∂⁴

// ∂⁵

// ∂⁶
