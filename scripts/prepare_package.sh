#!/usr/bin/env bash
# SPDX-License-Identifier: Apache-2.0
#
# Copyright (c) 2025 RTVLAS contributors

set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "${repo_root}"

./build.sh
./scripts/run_scenarios.sh
./scripts/summarize_evidence.py "${repo_root}"

rm -rf submission_package
mkdir -p submission_package
cp -R README.md LICENSE package_metadata.json package_manifest.json Cargo.toml Makefile build.sh core bindings tooling evidence proposal docs scenarios scripts submission_package/

echo "submission_package ready at ${repo_root}/submission_package"
