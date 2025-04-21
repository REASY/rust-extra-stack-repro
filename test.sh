#!/bin/sh

set -ex

cargo build --release

DEPS_PATH="${PWD}/${CARGO_TARGET_DIR}/release/deps"

c=$(find $DEPS_PATH -type f -name "rust_extra_stack-*.s" -print | xargs grep -A20 "fib2:" | grep -P 'sub\trsp, 4096' | wc -l)

if [ $c -eq 2 ]; then
  exit 1
fi


