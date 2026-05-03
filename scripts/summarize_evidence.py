#!/usr/bin/env python3
# SPDX-License-Identifier: Apache-2.0
#
# Copyright (c) 2025 RTVLAS contributors

from __future__ import annotations

import json
import subprocess
import sys
from datetime import datetime, timezone
from pathlib import Path


def expectation_for_mode(mode: str) -> str:
    return {
        "nominal": "No flags or rejects; trust should remain near 1.0.",
        "degraded": "Flags should appear without hard rejects; trust should decline but remain above failure.",
        "fault": "Reject behavior should appear with a deterministic first reject frame.",
    }[mode]


def evaluate(mode: str, scorecard: dict) -> tuple[bool, str]:
    accept = scorecard["accept_frames"]
    flag = scorecard["flag_frames"]
    reject = scorecard["reject_frames"]
    final_trust = scorecard["final_trust_score"]
    first_reject = scorecard["first_reject_frame"]

    if mode == "nominal":
        passed = reject == 0 and flag == 0 and final_trust >= 0.95
        observed = f"accept={accept}, flag={flag}, reject={reject}, final_trust={final_trust:.3f}"
    elif mode == "degraded":
        passed = reject == 0 and flag > 0 and 0.05 <= final_trust < 1.0
        observed = f"accept={accept}, flag={flag}, reject={reject}, final_trust={final_trust:.3f}"
    else:
        passed = reject > 0 and first_reject is not None
        observed = f"accept={accept}, flag={flag}, reject={reject}, first_reject={first_reject}, final_trust={final_trust:.3f}"
    return passed, observed


def git_head(repo_root: Path) -> str:
    proc = subprocess.run(
        ["git", "-C", str(repo_root), "rev-parse", "HEAD"],
        text=True,
        capture_output=True,
    )
    if proc.returncode != 0:
        return "UNCOMMITTED_SCAFFOLD"
    return proc.stdout.strip()


def main() -> int:
    repo_root = Path(sys.argv[1]).resolve() if len(sys.argv) > 1 else Path(__file__).resolve().parents[1]
    metadata = json.loads((repo_root / "package_metadata.json").read_text())
    manifest = json.loads((repo_root / "scenarios/manifest.json").read_text())
    evidence_root = repo_root / "evidence"

    generated_at = datetime.now(timezone.utc).replace(microsecond=0).isoformat().replace("+00:00", "Z")
    entries = []
    passed = 0
    for scenario in manifest:
        scorecard_path = repo_root / scenario["output"] / "trust_scorecard.json"
        scorecard = json.loads(scorecard_path.read_text())
        ok, observed = evaluate(scenario["mode"], scorecard)
        passed += int(ok)
        entries.append(
            {
                "id": scenario["id"],
                "label": scenario["label"],
                "mode": scenario["mode"],
                "expected": expectation_for_mode(scenario["mode"]),
                "observed": observed,
                "passed": ok,
                "scorecard_path": str(scorecard_path.relative_to(repo_root)),
                "timeline_path": str((repo_root / scenario["output"] / "timeline.json").relative_to(repo_root)),
                "proof_log_path": str((repo_root / scenario["output"] / "proof_log.txt").relative_to(repo_root)),
                "trace_path": str((repo_root / scenario["output"] / "trace.svg").relative_to(repo_root)),
                "accept_frames": scorecard["accept_frames"],
                "flag_frames": scorecard["flag_frames"],
                "reject_frames": scorecard["reject_frames"],
                "final_trust_score": scorecard["final_trust_score"],
                "first_reject_frame": scorecard["first_reject_frame"],
            }
        )

    summary = {
        "topic_id": metadata["topic_id"],
        "title": metadata["title"],
        "repo_name": metadata["repo_name"],
        "generated_at_utc": generated_at,
        "git_head": git_head(repo_root),
        "scenarios_total": len(entries),
        "scenarios_passed": passed,
        "pass_rate_pct": round((passed / max(1, len(entries))) * 100.0, 1),
        "evidence_type": "deterministic synthetic autonomy traces for submission-stage feasibility review",
        "entries": entries,
    }

    (repo_root / "package_manifest.json").write_text(json.dumps(summary, indent=2) + "\n")
    (evidence_root / "scorecard_summary.json").write_text(json.dumps(summary, indent=2) + "\n")

    lines = [
        "# Evidence Scorecard Summary",
        "",
        f"- Topic: `{metadata['topic_id']} {metadata['title']}`",
        f"- Generated: `{generated_at}`",
        f"- Git Head: `{summary['git_head']}`",
        f"- Scenario Pass Rate: `{summary['scenarios_passed']}/{summary['scenarios_total']} ({summary['pass_rate_pct']}%)`",
        f"- Evidence Type: `{summary['evidence_type']}`",
        "",
        "| Scenario | Mode | Result | Final Trust | First Reject | Scorecard |",
        "| --- | --- | --- | --- | --- | --- |",
    ]
    for entry in entries:
        result = "PASS" if entry["passed"] else "FAIL"
        first_reject = entry["first_reject_frame"] if entry["first_reject_frame"] is not None else "None"
        lines.append(
            f"| {entry['label']} | `{entry['mode']}` | `{result}` | `{entry['final_trust_score']:.3f}` | `{first_reject}` | [{entry['id']}]({entry['id']}/trust_scorecard.json) |"
        )
    lines.extend(
        [
            "",
            "## Notes",
            "",
            "- Nominal scenarios are expected to remain fully accepted.",
            "- Degraded scenarios are expected to produce concern signals without hard reject behavior.",
            "- Fault scenarios are expected to produce deterministic reject behavior.",
            "- This summary is generated automatically from the underlying per-scenario scorecards.",
            "",
        ]
    )
    (evidence_root / "scorecard_summary.md").write_text("\n".join(lines))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
