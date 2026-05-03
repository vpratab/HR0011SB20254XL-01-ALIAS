# Solicitation Alignment

## Topic Basis

- **Track posture:** Direct-to-Phase-II
- **Source basis:** DARPA ALIAS Missionized Autonomy for Emergency Services program page and FAQ, January 2026.
- **Objective summary:** Deliver a third-party autonomy application that integrates with ALIAS/MATRIX for emergency-service missions such as wildfire reconnaissance, suppression support, cargo sling load, or medevac-style operations.

## What This Repository Intentionally Covers

- missionized autonomy application behavior rather than core flight-control replacement
- integration with ALIAS/MATRIX concepts and AFSIM-style evaluation flows
- wildfire or emergency-response mission tasks with optional HMI and multi-aircraft coordination
- evidence that the mission app can remain safe and recoverable during degraded conditions

## How The Repository Maps To The Topic

| Solicitation Need | Repository Response |
| --- | --- |
| Topic-specific runtime checks | `core/src/profile.rs` encodes five topic-shaped trust properties tied to this mission area. |
| Repeatable proof and replay | `tooling/replay`, `tooling/eval`, `evidence/`, and `package_manifest.json` provide deterministic reproduction. |
| Integration path | `bindings/include/rt_vlas.h` and `bindings/src/lib.rs` define the C ABI boundary for autonomy-stack insertion. |
| Reviewer-verifiable evidence | `evidence/scorecard_summary.md`, `proof_log.txt`, `timeline.json`, and `trace.svg` make the behavior inspectable. |
| Clear scope discipline | This repository is scoped as: This repository is shaped as a mission-app assurance and review module that sits above autonomy middleware; it is not a replacement for low-level flight controls. |

## What The Package Is Not Claiming

- it is not a replacement for the underlying autonomy stack
- it is not a certification package
- it is not based on classified program data
- it is not claiming operational fielding approval

## Why The Current Shape Is Credible

The strongest near-term value of RTVLAS is the ability to make autonomy behavior observable,
explainable, and rejectable when it drifts outside mission or safety expectations. That is the
thread this repository follows for this specific topic.
