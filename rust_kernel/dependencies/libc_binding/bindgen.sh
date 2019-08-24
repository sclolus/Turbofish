#!/bin/bash
# Debian prerequis: sudo apt-get install llvm-3.9-dev libclang-3.9-dev clang-3.9
# cargo install bindgen
bindgen $1 --ctypes-prefix super --impl-debug --no-layout-tests --use-core --ignore-functions --blacklist-type u16 --blacklist-type u32 --blacklist-type u8 -- -target x86_64-pc-linux-gnu --sysroot /toolchain_turbofish/sysroot/
