#!/bin/bash
RUSTUP_VERSION="nightly-2019-02-10"
CARGO_XBUILD_VERSION="0.5.5"

rustup override set $RUSTUP_VERSION
rustup component add rust-src
res=`cargo install --list | grep "cargo-xbuild v$CARGO_XBUILD_VERSION"`
if [ -z "$res" ]
then
	cargo install --version $CARGO_XBUILD_VERSION cargo-xbuild --force
else
	echo "cargo-xbuild $CARGO_XBUILD_VERSION already installed"
fi
exit 0
