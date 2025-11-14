build:
    RUSTFLAGS="-Zlocation-detail=none -Zfmt-debug=none" cargo build -Z build-std=std,panic_abort -Z build-std-features= --release