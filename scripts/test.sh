#!/usr/bin/env bash
set -x
set -eo pipefail
cargo r -- data/test.ab && rustc ./data/output/out.rs  -o ./data/output/out.exe && ./data/output/out.exe 