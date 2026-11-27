//! Strapdown INS and error-state estimator.
//!
//! Navigation state, propagation, the filter, and measurement models are later
//! milestones. Linear algebra in this crate is [`nalgebra`] and nothing else.

// Pins the maths dependency before estimator code exists.
use nalgebra as _;
