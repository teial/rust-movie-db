check:
    cargo check

release:
    cargo build --release

serve:
    # Empty flags to prevent 'frameworks not supported' error: https://github.com/rust-lang/rust/issues/125534
    RUSTFLAGS="" dx serve
