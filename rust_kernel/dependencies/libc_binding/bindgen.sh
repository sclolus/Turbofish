#!/bin/bash
# Debian prerequis: sudo apt-get install llvm-3.9-dev libclang-3.9-dev clang-3.9
# cargo install bindgen

cd
echo '#![allow(non_camel_case_types)]'
bindgen $@ --rustified-enum Errno --no-layout-tests --use-core --ignore-functions  --  -nostdlib -fno-builtin  -fno-stack-protector -nodefaultlibs --sysroot ../../../toolchain_turbofish/sysroot/
