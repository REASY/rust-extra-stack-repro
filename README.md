This  repo is to reproduce and find commits that caused the regression [Why Rust compiler (1.77.0 to 1.85.0) reserves 2x extra stack for large enum?](https://internals.rust-lang.org/t/why-rust-compiler-1-77-0-to-1-85-0-reserves-2x-extra-stack-for-large-enum/22775/1)

# How to run
1. Install [cargo-bisect-rustc]() by running `cargo install cargo-bisect-rustc`
2. Run `cargo bisect-rustc --start 1.76.0 --end 1.77.0 --script ./test.sh  -vv -- build --release`