# Docs Index

This repository adapts RTVLAS for **HR0011SB20254XL-01 ALIAS Missionized Autonomy for Emergency Services**.

## Primary References

- [Architecture](architecture.md)
- [API](api.md)
- [Data Provenance](DATA_PROVENANCE.md)
- [Solicitation Alignment](../proposal/09_Solicitation_Alignment.md)
- [Submission Checklist](../proposal/10_Submission_Checklist.md)
- [Required Inputs](../proposal/11_Required_Inputs.md)
- [Technical Volume](../proposal/02_Technical_Volume.md)
- [Reviewer Guide](../proposal/04_Reviewer_Guide.md)

## Runtime Surface

- `core/src/model.rs`: snapshot schema, verdicts, property types
- `core/src/monitor.rs`: trust accumulation and frame evaluation
- `core/src/profile.rs`: topic-specific property set
- `core/src/logger.rs`: scorecard, proof log, and SVG trace generation
- `bindings/include/rt_vlas.h`: C ABI
- `tooling/replay/src/main.rs`: deterministic replay path
- `tooling/eval/src/main.rs`: scenario evaluator
- `tooling/viewer/index.html`: static reviewer viewer for scorecards and timelines


## Evidence Navigation

- [Evidence Guide](../evidence/README.md)
- [Evidence Summary](../evidence/scorecard_summary.md)
- [Scenario Manifest](../scenarios/manifest.json)
