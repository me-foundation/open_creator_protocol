#! /bin/bash

cargo build-bpf
cargo fmt && cargo clippy
cd sdk || exit
npm ci
npx solita
cd - || exit
