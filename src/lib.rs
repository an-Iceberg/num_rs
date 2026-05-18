#![allow(clippy::needless_return)]
#![allow(mixed_script_confusables)]
#![allow(non_snake_case)]
#![allow(dead_code)]

pub mod complex;
pub mod cry;
pub mod derivative;
pub mod gradient;
pub mod integral;
pub mod jacobian;
pub mod partial;
pub mod polynomial;

// TODO: restructure this akin to `numpy` and `scipy`.

/// Macro for syntactically more pleasing lambda functions/closures.
///
/// Preferably write it like this: `λ!{n => n*2}`.
///
/// ---
///
/// I would have loved to implement syntax that looks like this: `λ!{n -> n*2}`
/// but Rust doesn't allow such syntax :(
#[macro_export]
macro_rules! λ
{
  ( $($variable:ident $(: $type:ty)?),* => $expression:expr ) =>
  {
    |$($variable $(: $type)?),*| $expression
  }
}
