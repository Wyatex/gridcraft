//! GridCraft's calculation engine: the evaluator (references, names, tables, LET/LAMBDA,
//! dynamic arrays) and dependency-driven recalculation with spills and cycle detection.
#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::unimplemented, clippy::todo, clippy::unreachable)]
#![forbid(unsafe_code)]

#[cfg(feature = "eval")]
pub mod eval;
#[cfg(feature = "eval")]
mod pivotdata;
#[cfg(feature = "eval")]
mod precision;
#[cfg(feature = "eval")]
pub mod recalc;
#[cfg(feature = "eval")]
mod spill_index;
#[cfg(feature = "eval")]
mod trimrange;

#[cfg(feature = "eval")]
pub use eval::{Area, SPECIAL_FUNCTIONS, is_known_function};
#[cfg(feature = "eval")]
pub use precision::DisplayedPrecision;
#[cfg(feature = "eval")]
pub use recalc::{Bounded, Calc, Key, evaluate, evaluate_bounded, now_serial};

#[cfg(not(feature = "eval"))]
mod stub;
#[cfg(not(feature = "eval"))]
pub use stub::{Bounded, Calc, DisplayedPrecision, Key, eval, evaluate, evaluate_bounded, now_serial, recalc};
#[cfg(not(feature = "eval"))]
pub use stub::eval::{Area, SPECIAL_FUNCTIONS, is_known_function};

#[cfg(all(test, feature = "eval"))]
mod tests;
