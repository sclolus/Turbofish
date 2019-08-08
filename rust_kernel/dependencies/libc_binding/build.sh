#!/bin/bash

set -e
./bindgen.sh /toolchain_turbofish/sysroot/all_includes.h > src/libc.rs
