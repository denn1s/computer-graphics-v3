RUSTFLAGS="-A dead_code" cargo build --release && ./target/release/$(basename $(pwd))

