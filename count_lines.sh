#!/bin/bash
echo "C code"
find ./$1 -type f -name "*.c" -exec cat {} \; | sed '/^\s*$/d' | sed -e '/^\//d' | sed -e '/^\*\*/d' | sed -e '/^\*/d' | wc -l
echo "ASM code"
find ./$1 -type f -name "*.asm" -exec cat {} \; | sed '/^\s*$/d' | sed -e '/^\//d' | sed -e '/^\*\*/d' | sed -e '/^\*/d' | wc -l
echo "C headers"
find ./$1 -type f -name "*.h" -exec cat {} \; | sed '/^\s*$/d' | sed -e '/^\//d' | sed -e '/^\*\*/d' | sed -e '/^\*/d' | wc -l
echo "Rust code"
find ./$1 -type f -name "*.rs" -exec cat {} \; | sed '/^\s*$/d' | sed -e '/^\//d' | sed -e '/^\*\*/d' | sed -e '/^\*/d' | wc -l
exit 0

