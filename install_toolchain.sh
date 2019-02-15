#!/bin/bash

curl https://sh.rustup.rs -sSf | sh -s -- --default-host i686-unknown-linux-gnu --default-toolchain nightly
source $HOME/.cargo/env
rustup toolchain add nightly
rustup target add i686-unknown-linux-gnu
rustup component add rust-src
rustup component add rust-fmt
cargo install cargo-xbuild
sudo apt-get install qemu-system-x86 make gcc nasm
