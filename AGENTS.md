# AGENTS.md — aviato-nav

Public AVIATO Rust navigation-estimation engine: strapdown INS + error-state EKF, position and range aiding, and seeded Monte Carlo campaigns judged on **consistency (NEES/NIS)**, not just RMSE.

## Commands
- Build: `cargo build --workspace --all-targets`
- Test: `cargo test --workspace`
- Lint: `cargo clippy --workspace --all-targets -- -D warnings` and `cargo fmt --all --check`
- Docs: `cargo doc --workspace --no-deps`

## Layout (from M0.1)
- `aviato-nav-core/` estimator (state, propagation, EKF, measurement models) · `aviato-nav-sim/` truth trajectories, sensor models, experiment runner · `docs/` results

## Repo-specific rules
- Linear algebra: `nalgebra` only.
- State the frame and rotation convention (e.g. NED, Hamilton quaternions, active rotations) in every module that uses one.
- Every experiment is seeded and writes a run manifest (config hash, git rev, seed).
- **This repo is public.** Never paste private AVIATO material: no internal repo contents, fleet details, IPs, hardware serials, unpublished results or customer names.

## How tasks work
- Planning lives in the private hq repo (`iamjakkie/hq`). Each task here is an issue with **Context**, **Acceptance oracles**, **Allowed files** and a **Kind**; its first line links the hq original (`HQ copy: iamjakkie/hq#N`).
- Start from `gh issue view <n>`. Restate goal, oracles and allowed files, propose a plan, and wait for approval before editing.
- Behave according to **Kind**:
  - **Plumbing (agent)** – implement it, inside Allowed files only.
  - **Core (by hand)** – the human writes the logic. Do not write implementation code for it. Explain, point to references, review diffs, suggest test cases.
  - **Thinking** – produce the requested document; no code.
- Branch: `task/<n>-<short-slug>`. PR description: `Closes #<n>, closes iamjakkie/hq#<N>`, then what you verified (command + result) and what you did **not** verify. Tick only the oracles you actually checked.

## Always
- Run the build, tests and linters below before calling anything done.
- Keep the diff minimal and inside Allowed files.
- Cite the source (book/paper, section or equation) in a comment for any algorithm or equation.

## Ask first
- Adding or upgrading dependencies.
- Changing public APIs, feature flags or CI.
- Anything that needs a file outside Allowed files.

## Never
- Weaken, skip or delete a failing test to make CI pass. Edit existing tests only if the task's Allowed files include them.
- Edit files marked `// HAND-CORE` or `# HAND-CORE`.
- Commit secrets. LLM access goes through OpenRouter with the key in an env var (`OPENROUTER_API_KEY`).
