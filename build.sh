cargo build-bpf
cargo fmt && cargo clippy
cd sdk
yarn
yarn solita
cd -
