#!/bin/bash
CARGO_XBUILD_VERSION="0.5.5"

curl https://sh.rustup.rs -sSf | sh -s -- --default-host i686-unknown-linux-gnu --default-toolchain nightly
source $HOME/.cargo/env  
rustup toolchain add nightly
rustup target add i686-unknown-linux-gnu
rustup component add rust-src

res=`cargo install --list | grep "cargo-xbuild v$CARGO_XBUILD_VERSION"`
if [ -z "$res" ]
then
	cargo install --version $CARGO_XBUILD_VERSION cargo-xbuild --force
else
	echo "cargo-xbuild $CARGO_XBUILD_VERSION already installed"
fi
exit 0
