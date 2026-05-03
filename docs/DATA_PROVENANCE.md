# Data Provenance

This repository is a **readiness package** for **HR0011SB20254XL-01 ALIAS Missionized Autonomy for Emergency Services**.

## What The Current Evidence Uses

- deterministic autonomy-state scenarios generated locally in `scenarios/`
- topic-shaped trust inputs tuned to exercise the property framework
- repeatable evidence generation through `scripts/run_scenarios.sh`
- aggregate proof summaries through `scripts/summarize_evidence.py`

## What The Current Evidence Does Not Claim

- live flight qualification
- hardware certification
- operational deployment approval
- performance on classified or proprietary mission data

## Why This Evidence Is Appropriate At This Stage

The purpose of this package is to prove technical readiness, establish architecture, show repeatable evidence generation, and define a disciplined path to higher-fidelity validation against the target autonomy stack. This package does that
directly by coupling code, tests, scenario traces, scorecards, proof logs, and a machine-readable
package manifest.

## Integration Validation Gates

1. ingest representative autonomy logs from the intended platform class
2. validate the snapshot contract against a real integration boundary
3. expand scenario coverage with mission-program data and surrogate hardware
4. measure runtime overhead on target compute
5. compare trust outputs against operator or mission-engineering ground truth
