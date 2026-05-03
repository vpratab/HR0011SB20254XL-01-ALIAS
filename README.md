# HR0011SB20254XL-01 ALIAS Missionized Autonomy for Emergency Services

[![Topic](https://img.shields.io/badge/Topic-HR0011SB20254XL-01-0A3D62)](proposal/02_Technical_Volume.md)
[![Core](https://img.shields.io/badge/Core-Rust_Runtime_Monitor-1F618D)](core/src/lib.rs)
[![Bindings](https://img.shields.io/badge/ABI-C_Compatible-117A65)](bindings/include/rt_vlas.h)
[![Evidence](https://img.shields.io/badge/Evidence-Prebuilt-9A7D0A)](evidence/)
[![Package](https://img.shields.io/badge/Submission-Rebuildable-7C3AED)](scripts/prepare_package.sh)

This repository packages **RTVLAS** for **HR0011SB20254XL-01 ALIAS Missionized Autonomy for Emergency Services** as a **ALIAS/MATRIX mission-app assurance for emergency autonomy**.

> RTVLAS adapted as a wildfire and emergency-response mission-app assurance layer for ALIAS/MATRIX-style autonomy, monitoring whether missionized autonomous behaviors remain safe, timely, and recoverable during fireline reconnaissance and response operations.

**End product form:** Integrable software module plus evidence/review tooling that watches for unsafe or degraded emergency mission-app behavior and supports operator review via a lightweight web viewer.
**Solicitation track:** Direct-to-Phase-II

## Reviewer Start

- [Submission Index](proposal/00_Submission_Index.md)
- [Executive Summary](proposal/01_Executive_Summary.md)
- [Technical Volume](proposal/02_Technical_Volume.md)
- [Reviewer Guide](proposal/04_Reviewer_Guide.md)
- [Claim / Artifact Matrix](proposal/05_Claim_Artifact_Matrix.md)
- [Risk Register](proposal/07_Risk_Register.md)
- [Data Provenance](proposal/08_Data_Provenance.md)
- [Solicitation Alignment](proposal/09_Solicitation_Alignment.md)
- [Submission Checklist](proposal/10_Submission_Checklist.md)
- [Required Inputs](proposal/11_Required_Inputs.md)
- [Docs Index](docs/README.md)
- [Evidence Guide](evidence/README.md)
- [Evidence Summary](evidence/scorecard_summary.md)
- [Package Manifest](package_manifest.json)

## Why This Repo Exists

RTVLAS is not positioned here as the autonomy stack. It is positioned as the **runtime trust layer**
that independently monitors autonomy outputs, applies topic-specific safety and mission properties,
and emits structured evidence for operator review, recovery logic, and technical due diligence.

## Solicitation Focus This Repo Targets

- missionized autonomy application behavior rather than core flight-control replacement
- integration with ALIAS/MATRIX concepts and AFSIM-style evaluation flows
- wildfire or emergency-response mission tasks with optional HMI and multi-aircraft coordination
- evidence that the mission app can remain safe and recoverable during degraded conditions

## System Shape

```mermaid
flowchart LR
    A["Autonomy State Snapshot"] --> B["RTVLAS Monitor"]
    B --> C["Topic-Specific Property Set"]
    C --> D["Trust Verdict"]
    D --> E["Evidence Bundle"]
    D --> F["Integration Hooks"]
    E --> G["Reviewer Package"]
```

## Evidence Snapshot

| Scenario | Expected Outcome |
| --- | --- |
| [Nominal Fireline Recon Leg](evidence/scenario_01_nominal_response_leg/trust_scorecard.json) | Emergency autonomy executes a stable wildfire reconnaissance leg with healthy standoff, clearance margin, and recovery readiness. |
| [Smoke Corridor Compression](evidence/scenario_02_hazard_compression/trust_scorecard.json) | Fireline hazard margins and air-ground coordination begin to degrade but remain within recoverable limits. |
| [Unrecoverable Fireline Divert](evidence/scenario_03_unrecoverable_emergency_path/trust_scorecard.json) | Divert reachability, hazard margin, and mission-app recovery readiness collapse simultaneously, forcing a reject-grade mission assurance event. |

## One Command Rebuild

```bash
./scripts/prepare_package.sh
```

Rebuild output:

- regenerated `evidence/`
- regenerated `evidence/scorecard_summary.md` and `package_manifest.json`
- refreshed `submission_package/`
- rebuilt Rust workspace and tests

## Current Evidence Boundaries

This package is intentionally honest about maturity. Current evidence is based on deterministic,
topic-shaped autonomy traces generated inside this repository for repeatable feasibility or readiness review.
See [proposal/08_Data_Provenance.md](proposal/08_Data_Provenance.md) and [package_manifest.json](package_manifest.json).

## Repository Map

- [core/](core/): runtime monitor, property framework, evidence writer
- [bindings/](bindings/): C ABI for external autonomy stacks
- [tooling/](tooling/): replay, evaluation, and optional viewer tooling
- [evidence/](evidence/): pre-generated artifacts for all scenarios
- [proposal/](proposal/): reviewer-facing submission package
- [docs/](docs/): architecture and API references
- [scenarios/](scenarios/): deterministic input traces used to generate evidence
- [scripts/](scripts/): package rebuild and scenario execution
