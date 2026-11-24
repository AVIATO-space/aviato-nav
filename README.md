# aviato-nav

A small Rust navigation-estimation engine: IMU-driven state propagation, corrected by intermittent position and range observations, with reproducible Monte Carlo campaigns that show **when the estimate can be trusted and when it cannot** (NEES/NIS consistency, not just RMSE).

> Status: pre-alpha. Nothing to use yet. Milestones below.

## Scope

- Error-state EKF over a strapdown INS core
- Position aiding, range aiding to known anchors, outages and outlier gating
- Deterministic, seeded experiments with a run manifest
- Later: cooperative range aiding between vehicles

Linear algebra uses [`nalgebra`](https://nalgebra.rs).

## Milestones

| | Milestone |
|---|---|
| M0 | Workspace, CI, reuse audit |
| M1 | Synthetic truth + IMU error model + strapdown propagation |
| M2 | Error-state EKF + position aiding + consistency harness |
| M3 | Range aiding, outages, innovation gating |
| M4 | Batched campaigns |
| M5 | Cooperative range aiding |
| M6 | Write-up and v0.1 release |

## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or [MIT license](LICENSE-MIT) at your option.
