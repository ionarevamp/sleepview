#!/data/data/com.termux/files/usr/bin/bash

RUSTFLAGS="-C link-arg=-fuse-ld=gold" cargo run --release -- "$@"
