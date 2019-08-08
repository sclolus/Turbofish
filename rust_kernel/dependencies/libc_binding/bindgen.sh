#!/bin/bash
# Debian prerequis: sudo apt-get install llvm-3.9-dev libclang-3.9-dev clang-3.9
# cargo install bindgen

bindgen $@ --rustified-enum Errno --impl-debug --no-layout-tests --use-core --ignore-functions --blacklist-type u16 --blacklist-type u32 --blacklist-type u8 --  -nostdlib -fno-builtin  -fno-stack-protector -nodefaultlibs --sysroot /toolchain_turbofish/sysroot/
