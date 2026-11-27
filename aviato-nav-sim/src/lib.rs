//! Synthetic truth, sensor models, and the seeded experiment runner.
//!
//! Trajectory generators and the IMU error model are later milestones.
//! The `aviato-nav-sim` binary grows the `run` subcommand with the run manifest.

// The runner is built against the estimator crate.
use aviato_nav_core as _;
