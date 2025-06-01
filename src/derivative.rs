// https://en.wikipedia.org/wiki/Finite_difference_coefficient

// https://www.math.hkust.edu.hk/~mamu/courses/231/Slides/CH04_1B.pdf
// https://web.media.mit.edu/~crtaylor/calculator.html

/// Computes the first derivative d¹f/dx¹ of `f`
pub fn d<Function>(f: Function, x: f64, h: f64) -> f64
where Function: Fn(f64) -> f64
{
  return ((1./12.)*f(x-2.*h) - (2./3.)*f(x-h) + (2./3.)*f(x+h) + (1./12.)*f(x+2.*h)) / h;
}

/// Computes the first derivative d²f/dx² of `f`
pub fn d2(f: fn(f64)->f64, x: f64, h: f64) -> f64
{
  return d(|x: f64| d(f, x, h), x, h);
}

/// Computes the first derivative d³f/dx³ of `f`
pub fn d3(f: fn(f64)->f64, x: f64, h: f64) -> f64
{
  return d(|x: f64| d2(f, x, h), x, h);
}

/// Computes the first derivative d⁴f/dx⁴ of `f`
pub fn d4(f: fn(f64)->f64, x: f64, h: f64) -> f64
{
  return d(|x: f64| d3(f, x, h), x, h);
}

/*

// d¹

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
  return (-(1./60.)*f(x - 3.*h) + 0.15*f(x - 2.*h) - 0.75*f(x - h) + 0.75*f(x + h) - 0.15*f(x + 2.*h) + (1./60.)*f(x + 3.*h)) / h;
}

pub fn d1_8(f: fn(f64) -> f64, x: f64, h: f64) -> f64
{
  return ((1./280.)*f(x - 4.*h) - (4./105.)*f(x - 3.*h) + 0.2*f(x - 2.*h) - 0.8*f(x - h) + 0.8*f(x + h) - 0.2*f(x + 2.*h) + (4./105.)*f(x + 3.*h) - (1./280.)*f(x + 4.*h)) / h;
}

// d²

pub fn d2_2(f: fn(f64) -> f64, x: f64, h: f64) -> f64
{
  return (f(x - h) - 2.*f(x) + f(x + h)) / h.powi(2);
}

pub fn d2_4(f: fn(f64) -> f64, x: f64, h: f64) -> f64
{
  return (-(1./12.)*f(x - 2.*h) + (4./3.)*f(x - h) - 2.5*f(x) + (4./3.)*f(x + h) - (1./12.)*f(x + 2.*h)) / h.powi(2);
}

pub fn d2_6(f: fn(f64) -> f64, x: f64, h: f64) -> f64
{
  return ((1./90.)*f(x - 3.*h) - 0.15*f(x - 2.*h) + 1.5*f(x - h) - (49./18.)*f(x) + 1.5*f(x + h) - 0.15*f(x + 2.*h) + (1./90.)*f(x + 3.*h)) / h.powi(2);
}

pub fn d2_8(f: fn(f64) -> f64, x: f64, h: f64) -> f64
{
  return (-(1./560.)*f(x - 4.*h) + (8./315.)*f(x - 3.*h) - 0.2*f(x - 2.*h) + 1.6*f(x - h) - (205./72.)*f(x) + 1.6*f(x + h) - 0.2*f(x + 2.*h) + (8./315.)*f(x + 3.*h) - (1./560.)*f(x + 4.*h)) / h.powi(2);
}

// d³

pub fn d3_2(f: fn(f64) -> f64, x: f64, h: f64) -> f64
{
  return (-0.5*f(x - 2.*h) + f(x - h) - f(x + h) + 0.5*f(x + 2.*h)) / h.powi(3);
}

pub fn d3_4(f: fn(f64) -> f64, x: f64, h: f64) -> f64
{
  return (0.125*f(x - 3.*h) - f(x - 2.*h) + 1.625*f(x - h) - 1.625*f(x + h) + f(x + 2.*h) - 0.125*f(x + 3.*h)) / h.powi(3);
}

pub fn d3_6(f: fn(f64) -> f64, x: f64, h: f64) -> f64
{
  return (-(7./240.)*f(x - 4.*h) + 0.3*f(x - 3.*h) - (169./120.)*f(x - 2.*h) + (61./30.)*f(x - h) - (61./30.)*f(x + h) + (169./120.)*f(x + 2.*h) - 0.3*f(x + 3.*h) + (7./240.)*f(x + 4.*h)) / h.powi(3);
}

// d⁴

pub fn d4_2(f: fn(f64) -> f64, x: f64, h: f64) -> f64
{
  return (f(x - 2.*h) - 4.*f(x - h) + 6.*f(x) - 4.*f(x + h) + f(x + 2.*h)) / h.powi(4);
}

pub fn d4_4(f: fn(f64) -> f64, x: f64, h: f64) -> f64
{
  return (-(1./6.)*f(x - 3.*h) + 2.*f(x - 2.*h) - (13./2.)*f(x - h) + (28./3.)*f(x) - (13./2.)*f(x + h) + 2.*f(x + 2.*h) - (1./6.)*f(x + 3.*h)) / h.powi(4);
}

pub fn d4_6(f: fn(f64) -> f64, x: f64, h: f64) -> f64
{
  return ((7./240.)*f(x - 4.*h) - 0.4*f(x - 3.*h) + (169./60.)*f(x - 2.*h) - (122./15.)*f(x - h) + 11.375*f(x) - (122./15.)*f(x + h) + (169./60.)*f(x + 2.*h) - 0.4*f(x + 3.*h) + (7./240.)*f(x + 4.*h)) / h.powi(4);
}

// d⁵

pub fn d5_2(f: fn(f64) -> f64, x: f64, h: f64) -> f64
{
  return (-0.5*f(x - 3.*h) + 2.*f(x - 2.*h) - 2.5*f(x - h) + 2.5*f(x + h) - 2.*f(x + 2.*h) + 0.5*f(x + 3.*h)) / h.powi(5);
}

pub fn d5_4(f: fn(f64) -> f64, x: f64, h: f64) -> f64
{
  return ((1./6.)*f(x - 4.*h) - (3./2.)*f(x - 3.*h) + (13./3.)*f(x - 2.*h) - (29./6.)*f(x - h) + (29./6.)*f(x + h) - (13./3.)*f(x + 2.*h) + (3./2.)*f(x + 3.*h) - (1./6.)*f(x + 4.*h)) / h.powi(5);
}

pub fn d5_6(f: fn(f64) -> f64, x: f64, h: f64) -> f64
{
  return (-(13./288.)*f(x - 5.*h) + (19./36.)*f(x - 4.*h) - 2.71875*f(x - 3.*h) + 6.5*f(x - 2.*h) - (323./48.)*f(x - h) + (323./48.)*f(x + h) - 6.5*f(x + 2.*h) + 2.71875*f(x + 3.*h) - (19./36.)*f(x + 4.*h) + (13./288.)*f(x + 5.*h)) / h.powi(5);
}

// d⁶

pub fn d6_2(f: fn(f64) -> f64, x: f64, h: f64) -> f64
{
  return (f(x - 3.*h) - 6.*f(x - 2.*h) + 15.*f(x - h) - 20.*f(x) + 15.*f(x + h) - 6.*f(x + 2.*h) + f(x + 3.*h)) / h.powi(6);
}

pub fn d6_4(f: fn(f64) -> f64, x: f64, h: f64) -> f64
{
  return (-0.25*f(x - 4.*h) + 3.*f(x - 3.*h) - 13.*f(x - 2.*h) + 29.*f(x - h) - 37.5*f(x) + 29.*f(x + h) - 13.*f(x + 2.*h) + 3.*f(x + 3.*h) - 0.25*f(x + 4.*h)) / h.powi(6);
}

pub fn d6_6(f: fn(f64) -> f64, x: f64, h: f64) -> f64
{
  return ((13./240.)*f(x - 5.*h) - (19./24.)*f(x - 4.*h) + 5.4375*f(x - 3.*h) - 19.5*f(x - 2.*h) + 40.375*f(x - h) - 51.15*f(x) + 40.375*f(x - h) - 19.5*f(x - 2.*h) + 5.4375*f(x - 3.*h) - (19./24.)*f(x - 4.*h) + (13./240.)*f(x - 5.*h)) / h.powi(6);
}

*/
