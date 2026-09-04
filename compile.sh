#!/bin/bash
# ProgramBench-style build: must produce ./executable at the repo root.
set -e
cd "$(dirname "$0")"
cargo build --release -p melody_cli
cp "target/release/melody" executable
