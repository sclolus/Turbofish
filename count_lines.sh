#!/bin/bash
echo "fichier .c"
find ./$1 -type f -name "*.c" -exec cat {} \; | sed '/^\s*$/d' | sed -e '/^\//d' | sed -e '/^\*\*/d' | sed -e '/^\*/d' | wc -l
echo "fichier .asm"
find ./$1 -type f -name "*.asm" -exec cat {} \; | sed '/^\s*$/d' | sed -e '/^\//d' | sed -e '/^\*\*/d' | sed -e '/^\*/d' | wc -l
echo "fichier .h"
find ./$1 -type f -name "*.h" -exec cat {} \; | sed '/^\s*$/d' | sed -e '/^\//d' | sed -e '/^\*\*/d' | sed -e '/^\*/d' | wc -l
exit 0

