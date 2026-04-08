#!/usr/bin/env bash
set -euo pipefail
cd rust-core && cargo bench "$@"
