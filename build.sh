#!/usr/bin/env bash
# SPDX-License-Identifier: Apache-2.0
#
# Copyright (c) 2025 RTVLAS contributors

set -euo pipefail

. "${HOME}/.cargo/env"
cargo build --release --workspace
cargo test --workspace
