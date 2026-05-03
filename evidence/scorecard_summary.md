# Evidence Scorecard Summary

- Topic: `HR0011SB20254XL-01 ALIAS Missionized Autonomy for Emergency Services`
- Generated: `2026-05-03T04:55:03Z`
- Git Head: `6ff5f29501064bb974230493b79fa3090931edf6`
- Scenario Pass Rate: `3/3 (100.0%)`
- Evidence Type: `deterministic synthetic autonomy traces for submission-stage feasibility review`

| Scenario | Mode | Result | Final Trust | First Reject | Scorecard |
| --- | --- | --- | --- | --- | --- |
| Nominal Fireline Recon Leg | `nominal` | `PASS` | `1.000` | `None` | [scenario_01_nominal_response_leg](scenario_01_nominal_response_leg/trust_scorecard.json) |
| Smoke Corridor Compression | `degraded` | `PASS` | `0.427` | `None` | [scenario_02_hazard_compression](scenario_02_hazard_compression/trust_scorecard.json) |
| Unrecoverable Fireline Divert | `fault` | `PASS` | `0.000` | `20` | [scenario_03_unrecoverable_emergency_path](scenario_03_unrecoverable_emergency_path/trust_scorecard.json) |

## Notes

- Nominal scenarios are expected to remain fully accepted.
- Degraded scenarios are expected to produce concern signals without hard reject behavior.
- Fault scenarios are expected to produce deterministic reject behavior.
- This summary is generated automatically from the underlying per-scenario scorecards.
