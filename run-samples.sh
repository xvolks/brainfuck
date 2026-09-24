#!/usr/bin/env bash

run-samples() {
  cargo build 2> /dev/null && cargo build --release 2> /dev/null
  echo
  echo
  echo "+-------------------------------------------------------------------------+"
  for f in samples/*.bf; do
    echo "---------------  $f  -------------"
    echo "test" | ./target/release/bf --no-jit $f
    echo "------------------------------------------------"
  done
}

twiddle-default() {
    has_compress=$(grep -E 'default = \[(.+)\]' Cargo.toml)
    if [[ -z $has_compress ]]; then
        sed -Ee's/default = \[\]/default = ["compress"]/g' Cargo.toml > Cargo.toml.twiddle
    else
        sed -Ee's/default = \[(.+)\]/default = []/g' Cargo.toml > Cargo.toml.twiddle
    fi
    mv Cargo.toml.twiddle Cargo.toml
    cat Cargo.toml
}

run-samples
twiddle-default
run-samples
twiddle-default
