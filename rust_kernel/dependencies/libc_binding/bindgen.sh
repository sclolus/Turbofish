#!/bin/bash
# Debian prerequis: sudo apt-get install llvm-3.9-dev libclang-3.9-dev clang-3.9
# cargo install bindgen

cd
bindgen $@ --rustified-enum e_errno --no-layout-tests --use-core --ignore-functions  --  -nostdlib -fno-builtin  -fno-stack-protector -nodefaultlibs --sysroot cross/sysroot
