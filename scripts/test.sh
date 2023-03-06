#!/usr/bin/env bash
set -x
set -eo pipefail
cargo r -- data/test.ab  
(cd ./data/output; cargo r)
