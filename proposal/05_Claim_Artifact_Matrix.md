# Claim / Artifact Matrix

| Claim | Artifact |
| --- | --- |
| The repository contains a real runtime monitor | `core/src/monitor.rs` |
| Topic-specific trust properties are implemented | `core/src/profile.rs` |
| The monitor is test-covered | `core/tests/profile_tests.rs` |
| External autonomy stacks can integrate through a C ABI | `bindings/include/rt_vlas.h` and `bindings/src/lib.rs` |
| Deterministic replay is supported | `tooling/replay/src/main.rs` |
| Evidence artifacts are generated automatically | `tooling/eval/src/main.rs`, `scripts/run_scenarios.sh`, and `scripts/summarize_evidence.py` |
| The repository is explicitly tied back to the topic language | `proposal/09_Solicitation_Alignment.md` |
| The proposal claims are tied to concrete scenario outputs | `evidence/` |
| Data maturity and limits are explicitly disclosed | `proposal/08_Data_Provenance.md` and `docs/DATA_PROVENANCE.md` |
