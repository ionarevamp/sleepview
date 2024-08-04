#!/data/data/com.termux/files/usr/bin/bash

RUSTFLAGS="-C link-arg=-fuse-ld=gold" cargo build --release && \
time ($CARGO_TARGET_DIR/release/sleepview $@)
