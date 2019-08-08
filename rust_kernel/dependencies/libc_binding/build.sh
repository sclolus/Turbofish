#!/bin/bash

set -e
cd /
$CARGO_MANIFEST_DIR/bindgen.sh /toolchain_turbofish/sysroot/all_includes.h > $CARGO_MANIFEST_DIR/src/libc.rs
sed -i 's/::std::os::raw::/super::/g' $CARGO_MANIFEST_DIR/src/libc.rs
