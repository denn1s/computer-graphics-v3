#!/usr/bin/env bash
set -euo pipefail
cd -- "$(dirname -- "$0")"

if [[ "${1:-}" == "--gpu" ]]; then
    shift
    cmake -S gpu_demo -B target/gpu-demo -DCMAKE_BUILD_TYPE=Release
    cmake --build target/gpu-demo --parallel
    exec ./target/gpu-demo/shader_workshop "$@"
fi

exec cargo run --release -- "$@"
