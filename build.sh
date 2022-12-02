cargo build-bpf
cargo fmt && cargo clippy
cd sdk
npm ci
npx solita
cd -
