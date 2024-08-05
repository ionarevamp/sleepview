#!/data/data/com.termux/files/usr/bin/zsh -li

RUSTFLAGS="-C link-arg=-fuse-ld=mold" cargo build --release && \
RUSTFLAGS="-C link-arg=-fuse-ld=mold" cargo build && \
time ($CARGO_TARGET_DIR/release/sleepview $@) && \
la $CARGO_TARGET_DIR/*/sleepview
