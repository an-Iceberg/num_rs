pub fn d1_2(f: fn(f64) -> f64, x: f64, h: f64) -> f64
{
  return (-0.5*f(x-h) + 0.5*f(x+h)) / h;
}

pub fn d1_4(f: fn(f64) -> f64, x: f64, h: f64) -> f64
{
  return ((1./12.)*f(x-2.*h) - (2./3.)*f(x-h) + (2./3.)*f(x+h) + (1./12.)*f(x+2.*h)) / h;
}

pub fn d1_6(f: fn(f64) -> f64, x: f64, h: f64) -> f64
{
  todo!()
}

pub fn d1_8(f: fn(f64) -> f64, x: f64, h: f64) -> f64
{
  todo!()
}


pub fn d2_2() { }

pub fn d2_4() { }

pub fn d2_6() { }

pub fn d2_8() { }
