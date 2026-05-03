# Reviewer Guide

## 5-Minute Review

1. Read `proposal/01_Executive_Summary.md`
2. Read `proposal/03_Quad_Chart.md`
3. Read `evidence/scorecard_summary.md`
4. Open `evidence/scenario_01_nominal_response_leg/trust_scorecard.json`

## 20-Minute Deep Dive

1. Read `proposal/02_Technical_Volume.md`
2. Read `proposal/08_Data_Provenance.md`
3. Read `proposal/09_Solicitation_Alignment.md`
4. Read `proposal/05_Claim_Artifact_Matrix.md`
5. Inspect `core/src/model.rs`, `core/src/monitor.rs`, and `core/src/profile.rs`
6. Inspect `evidence/scenario_02_hazard_compression/timeline.json`
5. Open `tooling/viewer/index.html` and load the matching `trust_scorecard.json` and `timeline.json` files.
6. Review `core/tests/profile_tests.rs`

## Full Rebuild

```bash
./scripts/prepare_package.sh
```

Expected outputs:

- rebuilt Rust workspace
- passing tests
- regenerated evidence in `evidence/`
- packaged submission copy in `submission_package/`
